import { StickyObjectStatic, type NoteInfo, type IStickyObject, type IPosition } from '../types.js'
import { ScaleManager } from '../../managers/scaleManager.js'
import {  getAllNotePosition, getNoteId, saveNotePosition, deleteNote} from '../../tauriCommands.js';
import { StickyObjectFactory } from '../stickyObjectFactory.js';
export class StickyNote extends StickyObjectStatic implements IStickyObject{
   
  private static readonly memoTextClassName = ".text-sm";

    private isResizing = false;
    private isMoving = false;
    private resizeStartX = 0;
    private resizeStartY = 0;
    private moveStartX = 0;
    private moveStartY = 0;

    public element: HTMLElement;
    public static readonly className = 'sticky-note';
    constructor(id: string, left: string, top: string, width: string, height: string) { 
      super();
      this.element = this.createElement();
      this.element.id = id;
      this.element.style.left = left;
      this.element.style.top = top;
      this.element.style.width = width;
      this.element.style.height = height;
      this.addEventListeners();
    }
    public setDefaultPosition(): void {
      this.element.style.left = '50px';
      this.element.style.top = '50px';
      this.element.style.width = '200px';
      this.element.style.height = '200px';
    }
    public getNewStickyObjectId(): Promise<string> {
      return getNoteId();
    }
    static save(element: HTMLElement): void {
      const memo = (element.querySelector('.text-sm') as HTMLElement).textContent ?? ''
      saveNotePosition(element.id, element.style.left, element.style.top, element.style.width, element.style.height, memo);
      }
    static delete(id: string): void {
      deleteNote(Number.parseInt(id));
    }
    static async load(selectedFile: string): Promise<StickyNote[]> {
      const result: StickyNote[] = [];
            const noteInfos: NoteInfo[] = await getAllNotePosition(selectedFile);
            for (const noteInfo of noteInfos) {
              const noteInstance = StickyObjectFactory
              .load(StickyNote, noteInfo.id, noteInfo.left, noteInfo.top, noteInfo.width, noteInfo.height);
              (noteInstance.element.querySelector(StickyNote.memoTextClassName) as HTMLElement).textContent = noteInfo.memo;
              result.push(noteInstance);
            }
            
      return result;
    }

  onMoveMouseDown(e: MouseEvent): void {
    this.isMoving = true;
    const scale = ScaleManager.getInstance().getScale();
    this.moveStartX = e.clientX / scale - Number.parseFloat(this.element.style.left);
    this.moveStartY = e.clientY / scale - Number.parseFloat(this.element.style.top);
    e.stopPropagation(); 
  }
  onMoveMouseMove(e: MouseEvent): void {
    if (this.isMoving) {
      const scale = ScaleManager.getInstance().getScale();
      this.element.style.left = `${e.clientX / scale - this.moveStartX}px`;
      this.element.style.top = `${e.clientY / scale - this.moveStartY}px`;
    }
  }
  onMoveMouseUp(e: MouseEvent): void {
    if (this.isMoving) {
      this.isMoving = false;
    }
  }
  onResizeMouseDown(e: MouseEvent): void {
    this.isResizing = true;
    this.resizeStartX = e.clientX;
    this.resizeStartY = e.clientY;
    e.stopPropagation(); 
  }
  onResizeMouseMove(e: MouseEvent): void {
    if (this.isResizing) {
      const dx = e.clientX - this.resizeStartX;
      const dy = e.clientY - this.resizeStartY;
      this.element.style.width = `${Math.max(100, Number.parseFloat(this.element.style.width) + dx)}px`;
      this.element.style.height = `${Math.max(100, Number.parseFloat(this.element.style.height) + dy)}px`;
      this.resizeStartX = e.clientX;
      this.resizeStartY = e.clientY;
    }
  }
  onResizeMouseUp(e: MouseEvent): void {
    if (this.isResizing) {
      this.isResizing = false;
    }
  }

  getResizeHandle(): Element | null {
    return this.element.querySelector('.resize-handle');
  }
  getMoveHandle(): Element | null {
    return this.element.querySelector('.move-handle');
  }
  

    createElement() {
      const noteElement = document.createElement('div');
      noteElement.className = 'sticky-note bg-yellow-200 p-4 rounded-lg shadow-md max-w-none hover:shadow-lg transition-shadow';
      noteElement.style.position = 'absolute';
      const memoContent = '<p contenteditable="true" class="text-sm"></p>';
      noteElement.innerHTML = `
        <div class="move-handle"></div>
        <button class="delete" style="position: absolute; top: 5px; left: 5px; border: none; padding: 5px; cursor: pointer;">✖</button>
        ${memoContent}
        <div class="resize-handle"></div>
      `;
      return noteElement;
    }
  
    addEventListeners() {
      this.element.querySelector('.delete')?.addEventListener('click',  () => {
        this.element.remove();
      });
    }
  }