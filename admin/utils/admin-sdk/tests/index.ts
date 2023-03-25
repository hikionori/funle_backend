import axios from "axios";
import { baseUrl } from "../config";

export interface ChoiceTest {
    _id: { $oid: string };
    theme: string;
    question: string;
    answers: string[];
    answer: string;
    level: number;
}

export interface ActionTest {
    _id: { $oid: string };
    theme: string;
    question: string;
    answers: string[];
    answer: string;
    level: number;
}

export type Test = ChoiceTest | ActionTest;
export type TestType = "choice" | "action";
export interface AllTests {
    tests: ChoiceTest[];
    tests_with_actions: ActionTest[];
}

export class ChoiceTestBuilder implements ChoiceTest {
    _id!: { $oid: string };
    theme!: string;
    question!: string;
    answers!: string[];
    answer!: string;
    level!: number;

    public setId(id: string | "None") {
        // set "$oid" field
        this._id = { $oid: id };
    }

    public setTheme(theme: string) {
        this.theme = theme;
    }

    public setQuestion(question: string) {
        this.question = question;
    }

    public setAnswers(answers: string[]) {
        this.answers = answers;
    }

    public setAnswer(answer: string) {
        this.answer = answer;
    }

    public setLevel(level: number) {
        this.level = level;
    }

    // public getId() {
    //     return this._id.$oid;
    // }

    public getTheme() {
        return this.theme;
    }

    public getQuestion() {
        return this.question;
    }

    public getAnswers() {
        return this.answers;
    }

    public getAnswer() {
        return this.answer;
    }

    public getLevel() {
        return this.level;
    }

    public toJSON() {
        return {
            _id: this._id,
            theme: this.theme,
            question: this.question,
            answers: this.answers,
            answer: this.answer,
            level: this.level,
        };
    }
}

export class ActionTestBuilder implements ActionTest {
    _id!: { $oid: string };
    theme!: string;
    question!: string;
    answers!: string[];
    answer!: string;
    level!: number;

    public setId(id: string | "None") {
        this._id = { $oid: id };
    }

    public setTheme(theme: string) {
        this.theme = theme;
    }

    public setQuestion(question: string) {
        this.question = question;
    }

    public setAnswers(answers: string[]) {
        this.answers = answers;
    }

    public setAnswer(answer: string) {
        this.answer = answer;
    }

    public setLevel(level: number) {
        this.level = level;
    }

    public getId() {
        return this._id;
    }

    public getTheme() {
        return this.theme;
    }

    public getQuestion() {
        return this.question;
    }

    public getAnswers() {
        return this.answers;
    }

    public getAnswer() {
        return this.answer;
    }

    public getLevel() {
        return this.level;
    }

    public toJSON() {
        return {
            _id: this._id,
            theme: this.theme,
            question: this.question,
            answers: this.answers,
            answer: this.answer,
            level: this.level,
        };
    }
}

export const getAllTests = async (): Promise<AllTests> => {
    const response = await axios.get(`${baseUrl}/admin/get/tests/all`);
    return response.data;
};

export const getTestById = async (id: string): Promise<Test> => {
    const response = await axios.get(`${baseUrl}/admin/get/test?id=${id}`);
    return response.data;
};

export const createTest = (test_type: string, test: any) => {
    // Send POST request to create new test
    // url: /admin/create/test?test_type=<test_type>
    // body: json
    fetch(`${baseUrl}/admin/${test_type}/create/test`, {
        method: "POST",
        mode: "cors",
        body: JSON.stringify(test),
    });
};

//? // FIXME: updateTest() doesn't work
export const updateTest = async (
    test_type: TestType,
    test: any,
    id: string
) => {
    fetch(`${baseUrl}/admin/${test_type}/update/test?id=${id}`, {
        method: "PUT",
        mode: "cors",
        body: JSON.stringify(test),
    });
};
//? // FIXME: deleteTest() doesn't work
export const deleteTest = async (id: string, test_type: TestType) => {
    fetch(`${baseUrl}/admin/${test_type}/delete/test?id=${id}`, {
        method: "DELETE",
        mode: "cors",
    });
};
