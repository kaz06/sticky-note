import { BoardContentManager } from './boardContentManager.js'
import { ScaleManager } from './scaleManager.js'
import type { IDraggableAndScalable } from '../mouseEvent.js'

export class BoardManager implements IDraggableAndScalable{
    private board: HTMLElement;
    public boardContentManager: BoardContentManager;
  
    constructor() {
      this.board = document.getElementById('board') as HTMLElement;
      this.boardContentManager = new BoardContentManager(this.board);
      this.addEventListeners();
    }
    private async addEventListeners(): Promise<void> {
      await this.boardContentManager.populateTablePicker();
      this.boardContentManager.setupButtonListeners();
      this.setDragAndScale();
      document.body.style.backgroundColor =  "#888";

  }
  private isDragging = false;
  private startX = 0;
  private startY = 0;
  private posx = 0;
  private posy = 0;
  
  private updateTransform = () => {
    const scale = ScaleManager.getInstance().getScale();
    this.board.style.transform = `translate(${this.posx}px, ${this.posy}px) scale(${scale})`;
  };  

  onWheelEvent(e: WheelEvent): void {
    const scale = ScaleManager.getInstance().getScale();
    const { clientX, clientY, deltaY } = e;
    const newScale = deltaY < 0 ? scale * 1.1 : scale / 1.1;

    this.posx = clientX - (clientX - this.posx) * (newScale / scale);
    this.posy = clientY - (clientY - this.posy) * (newScale / scale);
    ScaleManager.getInstance().setScale(newScale);
    this.updateTransform();
    e.preventDefault();
  }
  onDragMouseDown(e: MouseEvent): void {
    if (!(e.target as Element).closest('.sticky-note')) {
      this.isDragging = true;
      this.startX = e.clientX;
      this.startY = e.clientY;
    }
  }
  onDragMouseMove(e: MouseEvent): void {
    if (this.isDragging) {
      this.posx += (e.clientX - this.startX);
      this.posy += (e.clientY - this.startY);
      this.updateTransform();
      this.startX = e.clientX;
      this.startY = e.clientY;
    }
  }
  onDragMouseUp(e: MouseEvent): void {
    this.isDragging = false;
  }

     setDragAndScale(): void {
      document.addEventListener('wheel', (e: WheelEvent) => {this.onWheelEvent(e)});
      document.addEventListener('mousedown', (e: MouseEvent) => {this.onDragMouseDown(e);});
      document.addEventListener('mousemove', (e: MouseEvent) => {this.onDragMouseMove(e);});
      document.addEventListener('mouseup', (e: MouseEvent) => { this.onDragMouseUp(e);});
    }
  }