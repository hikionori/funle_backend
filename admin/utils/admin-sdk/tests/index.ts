import axios from "axios";
import { baseUrl } from "../config";

export interface ChoiceTest {
    id: string | "None";
    theme: string;
    text_of_question: string;
    answers: string[];
    correct_answer: string;
    level: number;
}

export interface ActionTest {
    id: string | "None";
    theme: string;
    example: string;
    actions: string[];
    answer: string;
    level: number;
}

export type Test = ChoiceTest | ActionTest;
export type TestType = "choice" | "action";
export interface AllTests {
    choice: ChoiceTest[];
    action: ActionTest[];
}

export class ChoiceTestBuilder implements ChoiceTest {
    id!: string | "None";
    theme!: string;
    text_of_question!: string;
    answers!: string[];
    correct_answer!: string;
    level!: number;

    public setId(id: string | "None") {
        this.id = id;
    }

    public setTheme(theme: string) {
        this.theme = theme;
    }

    public setTextOfQuestion(text_of_question: string) {
        this.text_of_question = text_of_question;
    }

    public setAnswers(answers: string[]) {
        this.answers = answers;
    }

    public setCorrectAnswer(correct_answer: string) {
        this.correct_answer = correct_answer;
    }

    public setLevel(level: number) {
        this.level = level;
    }

    public getId() {
        return this.id;
    }

    public getTheme() {
        return this.theme;
    }

    public getTextOfQuestion() {
        return this.text_of_question;
    }

    public getAnswers() {
        return this.answers;
    }

    public getCorrectAnswer() {
        return this.correct_answer;
    }

    public getLevel() {
        return this.level;
    }
}

export class ActionTestBuilder implements ActionTest {
    id!: string | "None";
    theme!: string;
    example!: string;
    actions!: string[];
    answer!: string;
    level!: number;

    public setId(id: string | "None") {
        this.id = id;
    }

    public setTheme(theme: string) {
        this.theme = theme;
    }

    public setExample(example: string) {
        this.example = example;
    }

    public setActions(actions: string[]) {
        this.actions = actions;
    }

    public setAnswer(answer: string) {
        this.answer = answer;
    }

    public setLevel(level: number) {
        this.level = level;
    }

    public getId() {
        return this.id;
    }

    public getTheme() {
        return this.theme;
    }

    public getExample() {
        return this.example;
    }

    public getActions() {
        return this.actions;
    }

    public getAnswer() {
        return this.answer;
    }

    public getLevel() {
        return this.level;
    }
}

export const getAllTests = async(): Promise<AllTests> => {
    const response = await axios.get(`${baseUrl}/admin/get/tests/all`);
    return response.data;
}

export const getTestById = async(id: string): Promise<Test> => {
    const response = await axios.get(`${baseUrl}/admin/get/test?id=${id}`);
    return response.data;
}

export const createTest = async(test_type: TestType, test: Test) => {
    return axios.post(`${baseUrl}/admin/${test_type}/create/test`, test);
}

export const updateTest = async(test_type: TestType, test: Test, id: string) => {
    return axios.put(
        `${baseUrl}/admin/${test_type}/update/test?id=${id}`,
        test
    )
}

export const deleteTest = async(id: string) => {
    return axios.delete(`${baseUrl}/admin/delete/test?id=${id}`);
}