// 1:
export const ADD_SONG = 'ADD_SONG';

// 2:
export const addSongAction = () => ({
  type: ADD_SONG,
});

// 3:
export type AddSongAction = ReturnType<typeof addSongAction>;

// 4:
export type Action = AddSongAction;
