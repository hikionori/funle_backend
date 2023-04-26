import axios from "axios";
import { baseUrl } from "../config";
export interface Info {
    _id: {
        $oid: string;
    };
    title: string;
    theme: string;
    content_levels: [{ [key: number]: Content[] }];
}

export interface Content {
    content_type: string;
    data: string;
}

export class ContentBuilder implements Content {
    content_type!: string;
    data!: string;

    public setContentType(content_type: string) {
        this.content_type = content_type;
    }

    public setData(data: string) {
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
    _id!: {
        $oid: string;
    };
    title!: string;
    theme!: string;
    content_levels!: [{ [key: number]: Content[] }];

    public setId(id: string | "None") {
        this._id = {
            $oid: id,
        };
    }

    public setTitle(title: string) {
        this.title = title;
    }

    public setTheme(theme: string) {
        this.theme = theme;
    }

    public setContentLevels(content_levels: [{ [key: number]: Content[] }]) {
        this.content_levels = content_levels;
    }

    public getId() {
        return this._id;
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

export async function getInfoById(id: string) {
    const response = await axios.get(`${baseUrl}/admin/get/info?id=${id}`);
    return response.data;
}

export function getAllInfos() {
    return axios.get(`${baseUrl}/admin/get/info/all`).then((res) => res.data);
}

export function createInfo(info: any) {
    return axios.post(`${baseUrl}/admin/create/info`, info);
}

export function updateInfo(id: string, info: Info) {
    return axios.post(`${baseUrl}/admin/update/info?id=${id}`, info);
}

export function deleteInfo(id: string) {
    return axios.post(`${baseUrl}/admin/del/info?id=${id}`);
}
