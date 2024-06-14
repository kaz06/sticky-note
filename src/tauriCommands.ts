const GET_NOTE_ID_COMMAND = 'get_note_id_command';
const GET_HEADLINE_ID_COMMAND = 'get_headline_id_command';
const SAVE_NOTE_INFO_COMMAND = 'save_note_info_command';
const SAVE_HEADLINE_INFO_COMMAND = 'save_headline_info_command';
const GET_NOTE_INFO_COMMAND = 'get_note_info_command';
const GET_HEADLINE_INFO_COMMAND = 'get_headline_info_command';
const LOAD_NOTES_FROM_SCREEN_COMMAND = 'load_notes_from_screen_command';
const LOAD_HEADLINES_FROM_SCREEN_COMMAND = 'load_headlines_from_screen_command';
const LIST_SCREENS_COMMAND = 'list_screens_command';
const CREATE_SCREEN_COMMAND = 'create_screen_command';
const IS_BOARD_NAME_EXIST_COMMAND = 'is_board_name_exist_command';


// biome-ignore lint/complexity/noBannedTypes: <explanation>
// biome-ignore lint/suspicious/noExplicitAny: <explanation>
export async function invokeTauriCommand(command: string, args: Object = {}): Promise<any> {
    try {
      return await window.__TAURI__.tauri.invoke(command, args);
    } catch (error) {
      console.error('Failed to invoke Tauri command:', command, error);
      throw error;
    }
  }
  
export  async function getNoteId() {
    return invokeTauriCommand(GET_NOTE_ID_COMMAND);
  }

  export async function getHeadlineId() {
    return invokeTauriCommand(GET_HEADLINE_ID_COMMAND);
  }

  export  async function saveNotePosition(
    id: string, 
    left: string, 
    top: string, 
    width: string, 
    height: string, 
    memo: string): Promise<void> {
    await invokeTauriCommand(SAVE_NOTE_INFO_COMMAND, { id, left, top, width, height, memo});
  }
  
  export async function saveHeadlinePosition(
    id: string, 
    left: string, 
    top: string, 
    width: string, 
    height: string, 
    headline: string): Promise<void> {
    await invokeTauriCommand(SAVE_HEADLINE_INFO_COMMAND, { id, left, top, width, height, headline}); 
  }
  
  // biome-ignore lint/suspicious/noExplicitAny: <explanation>
    export  async function getNotePosition(id: string): Promise<any> {
    return await invokeTauriCommand(GET_NOTE_INFO_COMMAND, { id });
  }

  // biome-ignore lint/suspicious/noExplicitAny: <explanation>
  export async function getHeadlinePosition(id: string): Promise<any> {
    return await invokeTauriCommand(GET_HEADLINE_INFO_COMMAND, { id });
  }
  
  // biome-ignore lint/suspicious/noExplicitAny: <explanation>
    export  async function getAllNotePosition(board: string): Promise<any> {
    return invokeTauriCommand(LOAD_NOTES_FROM_SCREEN_COMMAND, { board });
  }
  
  // biome-ignore lint/suspicious/noExplicitAny: <explanation>
    export async function getAllHeadlinePosition(board: string): Promise<any> {
    return invokeTauriCommand(LOAD_HEADLINES_FROM_SCREEN_COMMAND, { board });
  }
  // biome-ignore lint/suspicious/noExplicitAny: <explanation>
    export function listScreens(): Promise<any> {
    return invokeTauriCommand(LIST_SCREENS_COMMAND);
  }
  
  
    export function isBoardNameExist(board: string): Promise<boolean> {
    return invokeTauriCommand(IS_BOARD_NAME_EXIST_COMMAND, { board });
  
  }
  // biome-ignore lint/suspicious/noExplicitAny: <explanation>
    export async function createNoteTable(board: string): Promise<any> {
    return invokeTauriCommand(CREATE_SCREEN_COMMAND, { board });
  }

  export async function deleteNote(id: number): Promise<void> {
    await invokeTauriCommand('delete_note_command', { id });
  }

  export async function deleteHeadline(id: number): Promise<void> {
    await invokeTauriCommand('delete_headline_command', { id });
  }