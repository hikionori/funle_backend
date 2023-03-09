import axios from "axios";
import { baseUrl } from "../config";
export interface Info {
  id: string | "None";
  title: string;
  theme: string;
  content_levels: { [key: number]: Content[] };
}

export interface Content {
  content_type: string;
  data: Uint8Array;
}

export class ContentBuilder implements Content {
    content_type!: string;
    data!: Uint8Array;
    
    public setContentType(content_type: string) {
        this.content_type = content_type;
    }
    
    public setData(data: Uint8Array) {
        this.data = data;
    }
    
    public getContentType() {
        return this.content_type;
    }
    
    public getData() {
        return this.data;
    }
}

export class InfoBuilder implements Info {
  id!: string;
  title!: string;
  theme!: string;
  content_levels!: { [key: number]: Content[] };

  public setId(id: string | "None") {
    this.id = id;
  }

  public setTitle(title: string) {
    this.title = title;
  }

  public setTheme(theme: string) {
    this.theme = theme;
  }

  public setContentLevels(content_levels: { [key: number]: Content[] }) {
    this.content_levels = content_levels;
  }

  public getId() {
    return this.id;
  }

  public getTitle() {
    return this.title;
  }

  public getTheme() {
    return this.theme;
  }

  public getContentLevels() {
    return this.content_levels;
  }
}

export function getInfoById(id: string) {
    return axios.get(`${baseUrl}/admin/get/info?id=${id}`)
}

export function getAllInfos() {
    return axios.get(`${baseUrl}/admin/get/info/all`)
}

export function createInfo(info: Info) {
    return axios.post(`${baseUrl}/admin/create/info`, info)
}

export function updateInfo(id: string, info: Info) {
    return axios.post(`${baseUrl}/admin/update/info?id=${id}`, info)
}

export function deleteInfo(id: string) {
    return axios.post(`${baseUrl}/admin/del/info?id=${id}`)
}