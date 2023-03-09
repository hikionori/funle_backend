import axios from "axios";
import { baseUrl } from "../config";
export interface Course {
    id: string;
    title: string;
    description: string;
    // Dictionary of levels
    levels: {
        [key: string]: Array<Level>;
    };
}

export class CourceBuilder implements Course{
    id!: string;
    title!: string;
    description!: string;
    levels!: { [key: string]: Level[]; };

    public setId(id:string) {
        this.id = id;
    }

    public setTitle(title: string) {
        this.title = title
    }

    public setDescription(description: string) {
        this.description = description
    }

    public setLevels(levels: {[key: string]: Level[]}) {
        this.levels = levels
    }

    public getId() {
        return this.id
    }

    public getDesription() {
        return this.description
    }

    public getTitle() {
        return this.title
    }

    public getLevels() {
        return this.levels
    }

}

export interface Level {
    id : string;
    title: string;
    mini_image: Array<Uint8Array>;
    type_: Type;
    // Optional fields
    n_of_tests: number | "None";
}

export enum Type {
    Info = "info",
    Test = "test"
}

export class LevelBuilder implements Level {
    id!: string;
    title!: string;
    mini_image!: Uint8Array[];
    type_!: Type;
    n_of_tests!: number | "None";

    public setId(id: string | "None") {
        this.id = id
    }

    public setTitle(title: string) {
        this.title = title
    }

    public setMiniImage(bytes: Uint8Array[]) {
        this.mini_image = bytes
    }

    public setType(type: Type) {
        this.type_ = type;
    }

    public setNOfTests(n_of_tests: number | "None") {
        this.n_of_tests = n_of_tests
    }
}

// Get all cources
export const getAllCources = async () => {
    const res: Array<Course> = await axios.get(`${baseUrl}/admin/get/cources/all`);
    return res;
}

// Get cource by id
export const getCourceById = async (id: string) => {
    const res: Course = await axios.get(`${baseUrl}/admin/get/cource?id=${id}`);
    return res;
}

// Create cource
export const createCource = async (cource: Course) => {
    const res: Course = await axios.post(`${baseUrl}/admin/add/cource`, cource);
    return res;
}

// Update cource
export const updateCource = async (id: string,cource: Course) => {
    const res = await axios.put(`${baseUrl}/admin/update/cource?id=${id}`, cource);
    return res;
}

// Delete cource

export const deleteCource = async (id: string) => {
    const res = await axios.delete(`${baseUrl}/admin/delete/cource?id=${id}`);
}