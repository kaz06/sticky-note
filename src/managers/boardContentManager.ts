import  type { IStickyObject,  Style } from '../stickyObjects/types.js'
import { StickyNote } from '../stickyObjects/models/stickyNote.js'
import { Headline } from '../stickyObjects/models/headline.js'
import {  listScreens} from '../tauriCommands.js';
import { StickyObjectFactory , type StickyObjectConstructor} from '../stickyObjects/stickyObjectFactory.js';
import Modal from './modal.js';
 export class BoardContentManager {
    screens: {
      frontScreen: HTMLElement | null;
      board: HTMLElement | null;
    };

    tablePicker: HTMLSelectElement;
    buttons: {
      createNote: HTMLElement | null;
      createHeadline: HTMLElement | null;
      load: HTMLElement | null;
      new: HTMLElement | null;
      save: HTMLElement | null;
    };

    stickyObjectContainer: HTMLElement | null;
  
    constructor(board: HTMLElement) {
      this.tablePicker = document.getElementById('sqlite-table-picker') as HTMLSelectElement;
      this.buttons = {
        createNote: document.getElementById('create-note'),
        createHeadline: document.getElementById('create-headline'),
        load: document.getElementById('load'),
        new: document.getElementById('new'),
        save: document.getElementById('save')
      };
      this.screens = {
        frontScreen: document.getElementById('front-screen'),
        board: board,
      };
      this.stickyObjectContainer = document.getElementById('sticky-object-container');
    }


    async populateTablePicker(): Promise<void> {
      while (this.tablePicker.firstChild) {
        this.tablePicker.removeChild(this.tablePicker.firstChild);
      }
      const tables = await listScreens();
      for (const table of tables) {
        const option = document.createElement('option');
        option.value = table;
        option.textContent = table;
        this.tablePicker.appendChild(option);
      }
    }
  
    setupButtonListeners(): void {
      this.buttons.createNote?.addEventListener('click', () => this.createStickyObject(StickyNote));
      this.buttons.createHeadline?.addEventListener('click', () => this.createStickyObject(Headline));
      this.buttons.save?.addEventListener('click', () => this.saveNotes());
      this.buttons.load?.addEventListener('click', () => this.loadNotes());
      this.buttons.new?.addEventListener('click', () => this.openModal());
    }
  

    async deleteNonExistentObjectsOnBoard(): Promise<void> {
      const selectedFile: string = this.tablePicker.value;
      const [stickyNotes, headlines] = await Promise.all([
        StickyNote.load(selectedFile),
        Headline.load(selectedFile)
      ]);
    
      const boardElementIds = (className: string) => 
        Array.from(this.stickyObjectContainer?.children || [])
          .filter((element) => element.classList.contains(className))
          .map((element) => element.id);
    
      const deleteNonExistentObjects = async (loadedObjects: IStickyObject[], boardIds: string[], deleteFunction: (id: string) => void) => {
        const loadedObjectIds = loadedObjects.map((obj) => obj.element.id);
        for (const id of loadedObjectIds) {
          if (!boardIds.includes(id)) {
            deleteFunction(id);
          }
        }
      };
    
      await Promise.all([
        deleteNonExistentObjects(stickyNotes, boardElementIds(StickyNote.className), StickyNote.delete),
        deleteNonExistentObjects(headlines, boardElementIds(Headline.className), Headline.delete)
      ]);
    }
    
    async saveNotes(): Promise<void> {
      await this.deleteNonExistentObjectsOnBoard();
    
      const saveMap = new Map<string, (element: HTMLElement) => void>([
        [StickyNote.className, StickyNote.save],
        [Headline.className, Headline.save]
      ]);
    
      for (const element of Array.from(this.stickyObjectContainer?.children || [])) {
        for (const [className, saveFunction] of saveMap) {
          if (element.classList.contains(className)) {
            saveFunction(element as HTMLElement);
            break;
          }
        }
      }
    }
    
    async loadNotes(): Promise<void> {
      this.modifyElementStyles(this.screens.frontScreen as HTMLElement, { top: '10px', right: '10px', left: 'auto', transform: 'none' });
      this.screens.board?.classList.remove('hidden');
      this.buttons.createNote?.classList.remove('hidden');
      this.buttons.createHeadline?.classList.remove('hidden');
      this.buttons.save?.classList.remove('hidden');
      this.tablePicker.style.right = '80px';
    
      const selectedFile: string = this.tablePicker.value;
      if (this.stickyObjectContainer) {
        this.stickyObjectContainer.innerHTML = '';
      }
    
      const [stickyNotes, headlines] = await Promise.all([
        StickyNote.load(selectedFile),
        Headline.load(selectedFile)
      ]);
    
      for (const stickyNote of stickyNotes) {
        this.stickyObjectContainer?.appendChild(stickyNote.element);
      }
      for (const headline of headlines) {
        this.stickyObjectContainer?.appendChild(headline.element);
      }
    }
    
    newSession(): void {
      this.modifyElementStyles(this.screens.frontScreen as HTMLElement, { top: '10px', right: '10px', left: 'auto', transform: 'none' });
      this.screens.board?.classList.remove('hidden');
      this.buttons.save?.classList.remove('hidden');
      this.modifyElementStyles(this.buttons.load as HTMLElement, { top: '10px', right: '10px', left: 'auto' });
      this.modifyElementStyles(this.tablePicker, { top: '10px', right: '80px', left: 'auto' });
    }
  
    async openModal() {
        Modal.openModal(() => this.populateTablePicker());
    }
    
  
    modifyElementStyles(element: HTMLElement, styles: Style): void {
      Object.assign(element.style, styles);
    }
    
    async createStickyObject(stickyObjcetType: StickyObjectConstructor<IStickyObject>) {
      const stickyObject = await StickyObjectFactory.create(stickyObjcetType);
      this.stickyObjectContainer?.appendChild(stickyObject.element);
    }

}



 