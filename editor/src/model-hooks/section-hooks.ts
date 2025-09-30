import {ID, INVALID_ID} from '../api/helpers';
import {useProject} from './project-hooks';

export const useSections = () => {
  const project = useProject();
  return project?.songs.flatMap((song) => song.sections);
};

export const useSectionsById = (sectionIds: ID[]) => {
  const sections = useSections();
  return sections?.filter((section) => sectionIds.includes(section.id));
};

export const useSectionById = (sectionId: ID) => {
  const sections = useSections();
  return sections?.find((section) => section.id.equals(sectionId));
};

export const useSelectedSectionId = () => {
  const project = useProject();
  return project?.selections?.section;
};

export const useSelectedSection = () => {
  const selectedSectionId = useSelectedSectionId();
  return useSectionById(selectedSectionId ?? INVALID_ID);
};
