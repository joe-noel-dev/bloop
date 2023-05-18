import {useContext} from 'react';
import {CoreDataContext} from '../core/CoreData';

export const useSections = () => {
  const {project} = useContext(CoreDataContext);
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
  const {project} = useContext(CoreDataContext);
  return project?.selections.section;
};

export const useSelectedSection = () => {
  const selectedSectionId = useSelectedSectionId();
  return useSectionById(selectedSectionId ?? '');
};

export const useSectionLength = (sectionId: string) => {
  const sections = useSections();
  let start: number | undefined = undefined;

  console.log('sections: ', sections);

  return (
    sections?.reduce<number | undefined>((length, section) => {
      if (length !== undefined) {
        return length;
      }

      if (start !== undefined) {
        const end = section.start;
        if (end >= start) {
          return end - start;
        }
      }

      if (section.id === sectionId) {
        console.log('start = ', section.start);
        start = section.start;
      }

      return undefined;
    }, undefined) ?? 0.0
  );
};
