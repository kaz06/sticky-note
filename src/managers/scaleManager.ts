export class ScaleManager {
    private static instance: ScaleManager;
    private scale = 1;
  
    private constructor() { }
  
    public static getInstance(): ScaleManager {
      if (!ScaleManager.instance) {
        ScaleManager.instance = new ScaleManager();
      }
      return ScaleManager.instance;
    }
  
    public setScale(newScale: number): void {
      this.scale = newScale;
    }
  
    public getScale(): number {
      return this.scale;
    }
  }