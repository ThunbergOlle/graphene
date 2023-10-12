import {
  createDir,
  exists,
  readTextFile,
  writeTextFile,
} from "@tauri-apps/api/fs";
import { BaseDirectory } from "@tauri-apps/api/path";

type ObjectLiteral = { [key: string]: unknown };

export class Store<T extends ObjectLiteral> {
  private store: T = {} as T;
  private fileName: string;

  constructor(fileName: string) {
    this.fileName = "stores/" + fileName + ".json";
    this.setup();
  }

  async setup() {
    const storeExists = await exists(this.fileName, {
      dir: BaseDirectory.AppData,
    });

    console.log("store exists", storeExists);

    if (!storeExists) {
      this.store = {} as T;
      await createDir("stores", {
        dir: BaseDirectory.AppData,
        recursive: true,
      });
      await writeTextFile(this.fileName, "{}", {
        dir: BaseDirectory.AppData,
      });
      return;
    }

    this.store = await this.read();
  }

  async get(key: keyof T, cacheBust = false): Promise<T[keyof T]> {
    if (!cacheBust) return this.store[key];

    const store = await this.read();
    this.store = store;
    return this.store[key];
  }

  set(key: keyof T, value: T[keyof T]) {
    this.store[key] = value;
    this.save();
  }

  update(data: Partial<T>) {
    this.store = { ...this.store, ...data };
    this.save();
  }

  delete(key: string) {
    delete this.store[key];
    this.save();
  }

  clear() {
    this.store = {} as T;
    this.save();
  }

  has(key: string) {
    return this.store[key] !== undefined;
  }

  getAll() {
    return this.store;
  }

  async save() {
    console.log("Saving store", this.fileName, this.store);
    return writeTextFile(this.fileName, JSON.stringify(this.store), {
      dir: BaseDirectory.AppData,
    });
  }

  async read(): Promise<T> {
    console.log("reading store");
    const file = await readTextFile(this.fileName, {
      dir: BaseDirectory.AppData,
    });
    if (!file) return {} as T;

    return JSON.parse(file);
  }
}

export const stores = {
  settings: new Store<{
    minecraftPath: string;
  }>("settings"),
};
