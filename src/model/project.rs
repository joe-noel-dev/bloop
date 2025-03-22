use super::id::ID;
use super::random_id;
use super::Song;
use super::INVALID_ID;
use crate::bloop::*;
use anyhow::anyhow;
use anyhow::Context;

impl ProjectInfo {
    pub fn empty() -> Self {
        Self {
            id: random_id(),
            name: "Project".to_string(),
            version: "1".to_string(),
            last_saved: 0,
            ..Default::default()
        }
    }
}

impl Project {
    pub fn empty() -> Self {
        let mut project = Self::new();
        project.info = Some(ProjectInfo::empty()).into();
        project.selections = Some(Selections::default()).into();
        project
    }

    pub fn with_name(mut self, name: String) -> Self {
        let info = self.info.as_mut().expect("Missing project info");
        info.name = name;
        self
    }

    pub fn with_songs(mut self, num_songs: usize, num_sections: usize) -> Self {
        assert!(num_songs >= 1);
        self.songs.clear();
        for _ in 0..num_songs {
            self = self.add_song(num_sections);
        }

        self = self.select_song_index(0);
        self
    }

    pub fn song_with_id(&self, id: ID) -> Option<&Song> {
        self.songs.iter().find(|s| s.id == id)
    }

    pub fn song_with_id_mut(&mut self, id: ID) -> Option<&mut Song> {
        self.songs.iter_mut().find(|s| s.id == id)
    }

    pub fn section_with_id(&self, id: ID) -> Option<&Section> {
        for song in self.songs.iter() {
            if let Some(section) = song.find_section(id) {
                return Some(section);
            }
        }

        None
    }

    pub fn replace_song(mut self, song: &Song) -> anyhow::Result<Self> {
        if !song.is_valid() {
            return Err(anyhow!("Invalid song"));
        }

        let old_song = match self.songs.iter_mut().find(|s| s.id == song.id) {
            Some(song) => song,
            None => return Err(anyhow!("Song not found")),
        };

        *old_song = song.clone();

        Ok(self)
    }

    pub fn add_section_to_song(self, song_id: ID) -> anyhow::Result<Self> {
        let song = self
            .song_with_id(song_id)
            .with_context(|| format!("Couldn't find song ID {song_id}"))?;

        let mut song = song.clone();

        let mut start = 0.0;

        if let Some(last_section) = song.sections.last() {
            let default_length = 16.0;
            start = last_section.start + default_length;
        }

        let section = Section::empty().with_start(start);

        song.sections.push(section);

        self.replace_song(&song)
    }

    pub fn contains_song(&self, song_id: ID) -> bool {
        self.songs.iter().any(|s| s.id == song_id)
    }

    pub fn selected_song_index(&self) -> Option<usize> {
        if self.selections.song == INVALID_ID {
            return None;
        }
        self.songs.iter().position(|song| song.id == self.selections.song)
    }

    pub fn song_with_index(&self, index: usize) -> Option<&Song> {
        self.songs.get(index)
    }

    pub fn select_song_index(self, song_index: usize) -> Self {
        let song_index = std::cmp::min(song_index, self.songs.len() - 1);

        let selected_song_id = match self.song_with_index(song_index) {
            Some(song) => song.id,
            None => {
                return self;
            }
        };

        self.select_song_with_id(selected_song_id)
    }

    pub fn select_song_with_id(mut self, song_id: ID) -> Self {
        if let Some(song) = self.song_with_id(song_id) {
            self.selections = Some(Selections {
                song: song_id,
                section: song.sections.first().map(|section| section.id).unwrap_or(INVALID_ID),
                ..Default::default()
            })
            .into();
        }

        self
    }

    pub fn song_with_section(&self, section_id: ID) -> Option<&Song> {
        self.songs
            .iter()
            .find(|song| song.sections.iter().any(|section| section.id == section_id))
    }

    pub fn remove_section(mut self, section_id: ID) -> anyhow::Result<Self> {
        let mut song = self
            .song_with_section(section_id)
            .with_context(|| format!("Couldn't find song with section ID: {section_id}"))?
            .clone();

        if song.sections.len() < 2 {
            return Err(anyhow!("Can't remove last section"));
        }

        let section_index = song.sections.iter().position(|section| section.id == section_id);

        song = song.remove_section(section_id);

        self = self.replace_song(&song)?;

        if !self.selection_is_valid() {
            self = match section_index {
                Some(index) => self.select_section_at_index(index)?,
                None => self.select_last_song(),
            }
        }

        if !self.is_valid() {
            return Err(anyhow!("Project is in an invalid state"));
        }

        Ok(self)
    }

    pub fn selected_song(&self) -> Option<&Song> {
        if self.selections.song == INVALID_ID {
            return None;
        }

        self.song_with_id(self.selections.song)
    }

    pub fn select_section_at_index(mut self, index: usize) -> anyhow::Result<Self> {
        let song = match self.selected_song() {
            Some(song) => song,
            None => return Err(anyhow!("No song selected")),
        };

        let index = index.min(song.sections.len() - 1);
        let new_section_id = song.sections[index].id;

        self = self.select_section(new_section_id)?;

        Ok(self)
    }

    pub fn selection_is_valid(&self) -> bool {
        let song_id = match self.selections.song {
            INVALID_ID => return false,
            song_id => song_id,
        };

        let section_id = match self.selections.section {
            INVALID_ID => return false,
            section_id => section_id,
        };

        let song = match self.song_with_id(song_id) {
            Some(song) => song,
            None => return false,
        };

        song.sections.iter().any(|section| section.id == section_id)
    }

    pub fn add_song(mut self, num_sections: usize) -> Self {
        assert!(num_sections >= 1);
        let sections: Vec<Section> = (0..num_sections).map(|_| Section::empty()).collect();
        let song = Song::empty().with_sections(sections);
        self.songs.push(song);
        self.select_last_song()
    }

    pub fn remove_song(mut self, song_id: ID) -> anyhow::Result<Self> {
        if self.songs.len() < 2 {
            return Err(anyhow!("Can't remove last song"));
        }

        if !self.contains_song(song_id) {
            return Err(anyhow!("Song ID not found to remove - {}", song_id));
        }

        let selected_song_index = self.selected_song_index();

        self.songs.retain(|song| song.id != song_id);

        if !self.selection_is_valid() {
            self = match selected_song_index {
                Some(index) => self.select_song_index(index),
                None => self.select_last_song(),
            };
        }

        Ok(self)
    }

    pub fn replace_section(mut self, new_section: &Section) -> anyhow::Result<Self> {
        if !new_section.is_valid() {
            return Err(anyhow!("Invalid section"));
        }

        self.songs
            .iter_mut()
            .filter_map(|song| song.find_section_mut(new_section.id))
            .for_each(|section| *section = new_section.clone());

        if !self.is_valid() {
            return Err(anyhow!("Project in an invalid state"));
        }

        Ok(self)
    }

    pub fn is_valid(&self) -> bool {
        self.songs.iter().all(|song| song.is_valid())
    }

    pub fn replace_sample(mut self, sample: &Sample) -> anyhow::Result<Self> {
        if !sample.is_valid() {
            return Err(anyhow!("Invalid sample"));
        }

        let old_sample = match self.find_sample_mut(sample.id) {
            Some(sample) => sample,
            None => return Err(anyhow!("Sample not found: {}", sample.id)),
        };

        *old_sample = sample.clone();

        self.songs
            .iter_mut()
            .filter(|song| song.sample.is_some() && song.sample.as_ref().unwrap().id == sample.id)
            .for_each(|song| song.tempo = sample.tempo.clone());

        Ok(self)
    }

    pub fn add_sample_to_song(mut self, sample: Sample, song_id: ID) -> anyhow::Result<Self> {
        let song = self
            .song_with_id_mut(song_id)
            .ok_or_else(|| anyhow!("Couldn't find song with ID: {}", song_id))?;

        let tempo = sample.tempo.clone();
        song.sample = Some(sample).into();
        song.tempo = tempo;

        Ok(self)
    }

    pub fn select_last_song(self) -> Self {
        let last_song_id = match self.songs.last() {
            Some(song) => song.id,
            None => return self,
        };

        self.select_song_with_id(last_song_id)
    }

    pub fn select_section(mut self, section_id: ID) -> anyhow::Result<Self> {
        if self.section_with_id(section_id).is_none() {
            return Err(anyhow!("Couldn't find section with ID: {}", section_id));
        }

        let song_id = self
            .song_with_section(section_id)
            .ok_or_else(|| anyhow!("Couldn't find song with Section ID: {}", section_id))?
            .id;

        self.selections = Some(Selections {
            song: song_id,
            section: section_id,
            ..Default::default()
        })
        .into();

        Ok(self)
    }

    #[allow(dead_code)]
    pub fn select_next_song(mut self) -> Self {
        let selected_song_index = match self.selected_song_index() {
            Some(index) => index,
            None => {
                return self;
            }
        };

        if selected_song_index < self.songs.len() - 1 {
            self = self.select_song_index(selected_song_index + 1)
        }

        self
    }

    #[allow(dead_code)]
    pub fn select_previous_song(mut self) -> Self {
        let selected_song_index = match self.selected_song_index() {
            Some(index) => index,
            None => {
                return self;
            }
        };

        if selected_song_index > 0 {
            self = self.select_song_index(selected_song_index - 1)
        }

        self
    }

    #[allow(dead_code)]
    pub fn select_next_section(self) -> anyhow::Result<Self> {
        let song_id = match self.selections.song {
            INVALID_ID => return Err(anyhow!("No song selected")),
            song_id => song_id,
        };

        let section_id = match self.selections.section {
            INVALID_ID => return Err(anyhow!("No section selected")),
            section_id => section_id,
        };

        let song = self
            .song_with_id(song_id)
            .ok_or_else(|| anyhow!("Couldn't find song with ID: {}", song_id))?;

        let current_section_index = match song.sections.iter().position(|section| section.id == section_id) {
            Some(position) => position,
            None => {
                return Ok(self);
            }
        };

        if current_section_index >= song.sections.len() - 1 {
            return Ok(self);
        }

        let next_section_id = song.sections[current_section_index + 1].id;
        self.select_section(next_section_id)
    }

    pub fn select_previous_section(self) -> anyhow::Result<Self> {
        let song_id = match self.selections.song {
            INVALID_ID => return Err(anyhow!("No song selected")),
            song_id => song_id,
        };

        let section_id = match self.selections.section {
            INVALID_ID => return Err(anyhow!("No section selected")),
            section_id => section_id,
        };

        let song = self
            .song_with_id(song_id)
            .ok_or_else(|| anyhow!("Couldn't find song with ID: {}", song_id))?;

        let current_section_index = match song.sections.iter().position(|section| section.id == section_id) {
            Some(position) => position,
            None => {
                return Ok(self);
            }
        };

        if current_section_index == 0 {
            return Ok(self);
        }

        let next_section_id = song.sections[current_section_index - 1].id;
        self.select_section(next_section_id)
    }

    pub fn remove_sample_from_song(mut self, song_id: ID) -> anyhow::Result<Self> {
        if let Some(song) = self.song_with_id_mut(song_id) {
            song.sample = None.into();
        }

        Ok(self)
    }

    pub fn find_sample(&self, sample_id: ID) -> Option<&Sample> {
        for song in self.songs.iter() {
            if let Some(sample) = song.sample.as_ref() {
                if sample.id == sample_id {
                    return Some(sample);
                }
            }
        }

        None
    }

    pub fn find_sample_mut(&mut self, sample_id: ID) -> Option<&mut Sample> {
        for song in self.songs.iter_mut() {
            if let Some(sample) = song.sample.as_mut() {
                if sample.id == sample_id {
                    return Some(sample);
                }
            }
        }

        None
    }

    pub fn replace_ids(mut self) -> Self {
        let info = self.info.as_mut().expect("Missing project info");
        info.id = random_id();

        self.songs = self.songs.iter().map(|song| song.clone().replace_ids()).collect();
        self.selections = Some(Selections {
            song: self.songs.first().map(|song| song.id).unwrap_or(INVALID_ID),
            section: self
                .songs
                .first()
                .and_then(|song| song.sections.first())
                .map(|section| section.id)
                .unwrap_or(INVALID_ID),
            ..Default::default()
        })
        .into();

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_with_songs() {
        let num_songs = 10;
        let num_sections = 10;
        let project = Project::empty().with_songs(num_songs, num_sections);
        assert_eq!(project.songs.len(), num_songs);
        assert!(project.songs.iter().all(|song| song.sections.len() == num_sections));
    }

    #[test]
    fn get_song_by_id() {
        let project = Project::empty().with_songs(5, 5);
        let song = &project.songs[2];
        let retrieved_song = match project.song_with_id(song.id) {
            Some(song) => song,
            None => panic!("Couldn't find song"),
        };
        assert_eq!(retrieved_song, song);
    }

    #[test]
    fn get_missing_song_by_id() {
        let project = Project::empty().with_songs(5, 5);
        let id = random_id();
        let retrieved_song = project.song_with_id(id);
        assert!(retrieved_song.is_none());
    }

    #[test]
    fn replace_song() {
        let mut project = Project::empty().with_songs(5, 5);
        let mut song = project.songs[3].clone();
        song.name = "New song name".to_string();
        project = project.replace_song(&song).expect("Couldn't replace song");
        assert_eq!(project.songs[3].name, "New song name");
    }

    #[test]
    fn select_next_song() {
        let mut project = Project::empty().with_songs(5, 5);
        let song_id = project.songs[1].id;
        project = project.select_next_song();
        assert_eq!(project.selections.song, song_id);
    }

    #[test]
    fn select_next_song_from_end() {
        let mut project = Project::empty().with_songs(5, 5);
        project = project.select_last_song();
        let song_id = project.songs[4].id;
        project = project.select_next_song();
        assert_eq!(project.selections.song, song_id);
    }

    #[test]
    fn select_previous_song() {
        let mut project = Project::empty().with_songs(5, 5);
        project = project.select_last_song();
        let song_id = project.songs[3].id;
        project = project.select_previous_song();
        assert_eq!(project.selections.song, song_id);
    }

    #[test]
    fn select_previous_song_from_start() {
        let mut project = Project::empty().with_songs(5, 5);
        let song_id = project.songs[0].id;
        project = project.select_previous_song();
        assert_eq!(project.selections.song, song_id);
    }

    #[test]
    fn select_next_section() {
        let mut project = Project::empty().with_songs(5, 5);
        let section_id = project.songs[0].sections[1].id;
        project = project.select_next_section().expect("Couldn't select next section");
        assert_eq!(project.selections.section, section_id);
    }

    #[test]
    fn select_previous_section() {
        let mut project = Project::empty().with_songs(5, 5);
        let initial_section_id = project.songs[0].sections[4].id;
        project = project
            .select_section(initial_section_id)
            .expect("Couldn't select initial section");
        project = project.select_previous_section().expect("Couldn't select next section");
        assert_eq!(project.selections.section, project.songs[0].sections[3].id);
    }
}
