import {useProject} from './project-hooks';

export const useSections = () => {
  const project = useProject();
  return project?.songs.flatMap((song) => song.sections);
};

export const useSectionsById = (sectionIds: string[]) => {
  const sections = useSections();
  return sections?.filter((section) => sectionIds.includes(section.id));
};

export const useSectionById = (sectionId: string) => {
  const sections = useSections();
  return sections?.find((section) => section.id === sectionId);
};

export const useSelectedSectionId = () => {
  const project = useProject();
  return project?.selections.section;
};

export const useSelectedSection = () => {
  const selectedSectionId = useSelectedSectionId();
  return useSectionById(selectedSectionId ?? '');
};
