import  type { IMovable, IResizable } from "../mouseEvent";

export interface IPosition {
    id: string;
    left: string;
    top: string;
    width: string;
    height: string;
  }

  export interface IStickyObject extends IResizable, IMovable {
    element: HTMLElement;
    getResizeHandle(): Element | null;
    getMoveHandle(): Element | null;
    setDefaultPosition(): void;
    getNewStickyObjectId: () => Promise<string>;
  }
  // biome-ignore lint/complexity/noStaticOnlyClass: <explanation>
    export abstract class StickyObjectStatic {
      static readonly className: string;
      static save(element: HTMLElement): void {
      }
      static async load(selectedFile: string): Promise<IStickyObject[]> {
        return [];
      }

      static delete(id: string): void {
        
      }
    }
  export interface Style {
    top?: string;
    right?: string;
    left?: string;
    transform?: string;
  }
export class NoteInfo {
    id: string;
    left: string;
    top: string;
    width: string;
    height: string;
    boardId: string;
    memo: string;
    constructor(
      id: string, 
      left: string, 
      top: string, 
      width: string, 
      height: string, 
      boardId: string, 
      memo: string) {
      this.id = id;
      this.left = left;
      this.top = top;
      this.width = width;
      this.height = height;
      this.boardId = boardId;
      this.memo = memo;
    }
  }

  export class HeadlineInfo {
    id: string;
    left: string;
    top: string;
    width: string;
    height: string;
    boardId: string;
    headline: string;
    constructor(
      id: string, 
      left: string, 
      top: string, 
      width: string, 
      height: string, 
      boardId: string, 
      headline: string) {
      this.id = id;
      this.left = left;
      this.top = top;
      this.width = width;
      this.height = height;
      this.boardId = boardId;
      this.headline = headline;
    }
  }