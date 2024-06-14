import { createNoteTable, isBoardNameExist } from '../tauriCommands.js';

// biome-ignore lint/complexity/noStaticOnlyClass: <explanation>
class Modal {
  static openModal(onOkCallback: () => void): void {
    if (document.getElementById('modal-overlay') || document.getElementById('modal-box')) {
      return;
    }

    const overlay = document.createElement('div');
    overlay.className = 'overlay';
    overlay.id = 'modal-overlay';

    const blocker = document.createElement('div');
    blocker.className = 'blocker';
    blocker.id = 'modal-blocker';

    const modal = document.createElement('div');
    modal.className = 'modal';
    modal.id = 'modal-box';
    modal.innerHTML = `
      <div class="modal-content">
        <div class="modal-error">
        </div>
        <input type="text" id="modal-input" class="bg-gray-100 border border-gray-300 rounded py-2 px-3">
        <button id="modal-ok" class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded mt-2">OK</button>
        <button id="modal-cancel" class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded mt-2">Cancel</button>
      </div>
    `;

    document.body.appendChild(overlay);
    document.body.appendChild(blocker);
    document.body.appendChild(modal);

    blocker.addEventListener('wheel', Modal.blockEvent, { passive: false });
    blocker.addEventListener('mousedown', Modal.blockEvent, true);
    blocker.addEventListener('mousemove', Modal.blockEvent, true);
    blocker.addEventListener('mouseup', Modal.blockEvent, true);

    
    document.getElementById('modal-ok')?.addEventListener('click', async () => {
      const inputVaule = (document.getElementById('modal-input') as HTMLInputElement).value;
      const result = await Modal.isErrorInput(inputVaule);
      if(result[0]){
        Modal.showErrorMessage(result[1]);
      }else{
        Modal.createBoard();
        Modal.closeModal();
        onOkCallback();
      }
    });

    document.getElementById('modal-cancel')?.addEventListener('click', () => {
      Modal.closeModal();
    });

    modal.addEventListener('wheel', (e) => e.stopPropagation(), { passive: false });
    modal.addEventListener('mousedown', (e) => e.stopPropagation(), true);
    modal.addEventListener('mousemove', (e) => e.stopPropagation(), true);
    modal.addEventListener('mouseup', (e) => e.stopPropagation(), true);
  }

  static async createBoard(): Promise<void> {
    await createNoteTable((document.getElementById('modal-input') as HTMLInputElement).value);
  }

  static  closeModal(): void {
    
    const overlay = document.getElementById('modal-overlay');
    const modal = document.getElementById('modal-box');
    const blocker = document.getElementById('modal-blocker');

    if (overlay && modal && blocker) {
      document.body.removeChild(overlay);
      document.body.removeChild(modal);
      document.body.removeChild(blocker);
    }

    blocker?.removeEventListener('wheel', Modal.blockEvent);
    blocker?.removeEventListener('mousedown', Modal.blockEvent, true);
    blocker?.removeEventListener('mousemove', Modal.blockEvent, true);
    blocker?.removeEventListener('mouseup', Modal.blockEvent, true);
  }

  static blockEvent(e: Event): void {
    e.stopImmediatePropagation();
    e.preventDefault();
  }

  static async isErrorInput(inputValue: string): Promise<[boolean, string]>{
     if (inputValue === ''){
       return [true, 'no input value'];
     }
     const result = await isBoardNameExist(inputValue);
      if(result){
        return [true, 'board name already exist'];
      }
        return [false, ''];
  }

  static showErrorMessage(inputErrorMessage: string): void{
    const modalError = document.querySelector('.modal-error') as HTMLDivElement;
    modalError.innerHTML = `<p>${inputErrorMessage}</p>`;
  }

}

  export default Modal;