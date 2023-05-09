import {useContext} from 'react';
import {CoreDataContext} from '../core/CoreData';

export const useSections = () => {
  const {project} = useContext(CoreDataContext);
  return project?.sections;
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
  const {project} = useContext(CoreDataContext);
  return project?.selections.section;
};

export const useSelectedSection = () => {
  const selectedSectionId = useSelectedSectionId();
  return useSectionById(selectedSectionId ?? '');
};
