
import type { IStickyObject, IPosition } from './types.js';


export type StickyObjectConstructor<T extends IStickyObject> = new (
  id: string, left: string, top: string, width: string, height: string) => T;

// biome-ignore lint/complexity/noStaticOnlyClass: <explanation>
export class StickyObjectFactory {

    static load<T extends IStickyObject>(
      ctor: StickyObjectConstructor<T>, id: string, left: string, top: string, width: string, height: string): T {
      const instance = new ctor(id, left, top, width, height);
      StickyObjectFactory.setMouseEvents(instance);
      return instance;
    }

    static async create<T extends IStickyObject>(ctor: StickyObjectConstructor<T>): Promise<T> {
      const instance = new ctor('-1', '0px', '0px', '0px', '0px');
      instance.setDefaultPosition();
      instance.element.id = await instance.getNewStickyObjectId();
      StickyObjectFactory.setMouseEvents(instance);
      return instance;
    }

    private static setMouseEvents<T extends IStickyObject>(instance: T): void {
      const resizeHandle = instance.getResizeHandle();
      const moveHandle = instance.getMoveHandle();

      if(resizeHandle){
        (resizeHandle as HTMLElement).addEventListener('mousedown', (e: MouseEvent) => { instance.onResizeMouseDown(e); });
      }
      if(moveHandle){
        (moveHandle as HTMLElement).addEventListener('mousedown', (e: MouseEvent) => { instance.onMoveMouseDown(e); });  
      }

      document.addEventListener('mousemove', (e: MouseEvent) => { 
          instance.onMoveMouseMove(e);
          instance.onResizeMouseMove(e);
      });
      
      document.addEventListener('mouseup', (e: MouseEvent) => { 
          instance.onMoveMouseUp(e);
          instance.onResizeMouseUp(e);
      });
    }
    
}