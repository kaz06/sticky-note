export interface IResizable {

    onResizeMouseDown(e: MouseEvent): void;
    onResizeMouseMove(e: MouseEvent): void;
    onResizeMouseUp(e: MouseEvent): void;
  }
  
export interface IMovable {  
    onMoveMouseDown(e: MouseEvent): void;
    onMoveMouseMove(e: MouseEvent): void;
    onMoveMouseUp(e: MouseEvent): void;
}
  
export interface IDraggableAndScalable {
    onWheelEvent(e: WheelEvent): void;
    onDragMouseDown(e: MouseEvent): void;
    onDragMouseMove(e: MouseEvent): void;
    onDragMouseUp(e: MouseEvent): void;
}