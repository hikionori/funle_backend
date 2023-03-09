import axios from "axios";
import { baseUrl } from "../config";

export type UserRole = "User" | "Student" | "Teacher";

export interface UserProgress {
    cources: string[];
    tests: string[];
    infos: string[];
}

export interface User {
    id: string | "None";
    username: string;
    email: string;
    hashed_password: string;
    role: UserRole;
    progress: UserProgress;
}

export class UserBuilder implements User {
    id!: string | "None";
    username!: string;
    email!: string;
    hashed_password!: string;
    role!: UserRole;
    progress!: UserProgress;

    public setId(id: string | "None") {
        this.id = id;
    }

    public setUsername(username: string) {
        this.username = username;
    }

    public setEmail(email: string) {
        this.email = email;
    }

    public setHashedPassword(hashed_password: string) {
        this.hashed_password = hashed_password;
    }

    public setRole(role: UserRole) {
        this.role = role;
    }

    public setProgress(progress: UserProgress) {
        this.progress = progress;
    }
}

export const deleteUser = async (id: string) => {
    axios.delete(`${baseUrl}/admin/del/user?id=${id}`);
}

export const getAllUsers = async (): Promise<User[]> => {
    const response: User[] = await axios.get(`${baseUrl}/admin/get/users`);
    return response;
}

export const getUserById = async (id: string): Promise<User> => {
    const response: User = await axios.get(`${baseUrl}/admin/get/user?id=${id}`);
    return response;
}

export const updateUser = async (user: User, id: string) => {
    axios.put(`${baseUrl}/admin/update/user?id=${id}`, user);
}

// User progress functions
export const updateUserProgress = async (progress: UserProgress, id: string) => {
    axios.put(`${baseUrl}/admin/update/user/progress?id=${id}`, progress);
}

export type JoiningData = {
    cource_id: string;
    user_id: string;
}

export type TestPassingData = {
    test_id: string;
    user_id: string;
}

export type InfoPassingData = {
    info_id: string;
    user_id: string;
}

export const addCourceToUserProgress = async (data: JoiningData) => {
    axios.put(`${baseUrl}/admin/add/cource/user`, data);
}

export const removeCourceFromUserProgress = async (data: JoiningData) => {
    axios.put(`${baseUrl}/admin/remove/cource/user`, data);
}

export const addTestToUserProgress = async (data: TestPassingData) => {
    axios.put(`${baseUrl}/admin/add/test/user`, data);
}

export const removeTestFromUserProgress = async (data: TestPassingData) => {
    axios.put(`${baseUrl}/admin/remove/test/user`, data);
}

export const addInfoToUserProgress = async (data: InfoPassingData) => {
    axios.put(`${baseUrl}/admin/add/info/user`, data);
}

export const removeInfoFromUserProgress = async (data: InfoPassingData) => {
    axios.put(`${baseUrl}/admin/remove/info/user`, data);
}