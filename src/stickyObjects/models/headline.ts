import { type HeadlineInfo, type IPosition, StickyObjectStatic } from '../types.js'
import { ScaleManager } from '../../managers/scaleManager.js'
import type { IStickyObject } from '../types.js';
import { StickyObjectFactory } from '../stickyObjectFactory.js';
import { getAllHeadlinePosition, getHeadlineId, saveHeadlinePosition, deleteHeadline } from '../../tauriCommands.js';
import type { IDraggableAndScalable } from '../../mouseEvent.js';
export class Headline extends StickyObjectStatic implements IStickyObject, IDraggableAndScalable{

    private isResizing = false;
    private isMoving = false;
    private resizeStartX = 0;
    private resizeStartY = 0;
    private moveStartX = 0;      
    private moveStartY = 0;
    private static headlineTextClassName = '.text-sm';

    public element: HTMLElement;

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
    this.element.style.left = '100px';
    this.element.style.top = '100px';
    this.element.style.width = '500px';
    this.element.style.height = '200px';
  }
    
    public getNewStickyObjectId(): Promise<string> {
      return getHeadlineId();
    }
    public static readonly className = 'headline';
    public static save(element: HTMLElement): void {
      const headline = (element.querySelector(Headline.headlineTextClassName) as HTMLElement).textContent ?? ''
      saveHeadlinePosition(element.id, element.style.left, element.style.top, element.style.width, element.style.height, headline); 
    }
    public static async load(selectedFile: string): Promise<Headline[]> { 
      const result: Headline[] = [];
            const headlineInfos: HeadlineInfo[] = await getAllHeadlinePosition(selectedFile);
            for (const headlineInfo of headlineInfos) {
              const headlineInstance = StickyObjectFactory
              .load(Headline, headlineInfo.id, headlineInfo.left, headlineInfo.top, headlineInfo.width, headlineInfo.height);
              (headlineInstance.element.querySelector(Headline.headlineTextClassName) as HTMLElement).textContent = headlineInfo.headline;
              result.push(headlineInstance);
            }
      return result;
    }
    public static async delete(id: string): Promise<void> {
      await deleteHeadline(Number.parseInt(id));
    }
    

    
  onWheelEvent(e: WheelEvent): void {
    const scale = ScaleManager.getInstance().getScale();
    const delta = e.deltaY > 0 ? -0.1 : 0.1;
    const newScale = scale + delta;
    const fontsize = 50 + Math.round(1 / newScale);
    const headlineMessage = this.element.querySelector(Headline.headlineTextClassName);
    (headlineMessage as HTMLElement).style.fontSize = `${fontsize}px`;

  }
  onDragMouseDown(e: MouseEvent): void {

  }
  onDragMouseMove(e: MouseEvent): void {
   
  }
  onDragMouseUp(e: MouseEvent): void {
    
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
      noteElement.className = 'headline bg-blue-200 p-4 rounded-lg shadow-md max-w-none hover:shadow-lg transition-shadow';
      noteElement.style.position = 'absolute';
      const headlineText = '<p contenteditable="true" class="text-sm"></p>';
      noteElement.innerHTML = `
        <div class="move-handle"></div>
        <button class="delete" style="position: absolute; top: 5px; left: 5px; border: none; padding: 5px; cursor: pointer;">✖</button>
        ${headlineText}
        <div class="resize-handle"></div>
      `;
      const message = noteElement.querySelector(Headline.headlineTextClassName);
      (message as HTMLElement).style.fontSize = '50px';
      const resizeHandle = noteElement.querySelector('.resize-handle') as HTMLElement;
      if (resizeHandle) {
        resizeHandle.style.borderWidth = '20px 20px 0 0';
        resizeHandle.style.borderColor = 'transparent #ADD8E6 transparent transparent';
      }
      return noteElement;
    }
  
    addEventListeners() {
      this.element.querySelector('.delete')?.addEventListener('click',  () => {
        this.element.remove();
      });
      document.addEventListener('wheel', (e: WheelEvent) => {this.onWheelEvent(e)});
    }
  }