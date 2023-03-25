import {
    AbsoluteCenter,
    Center,
    HStack,
    Input,
    NumberInput,
    NumberInputField,
    Radio,
    RadioGroup,
} from "@chakra-ui/react";
import Head from "next/head";
import { useRouter } from "next/router";
import { useCallback, useEffect, useState } from "react";
import { FaPlus, FaUpload } from "react-icons/fa";
import AddOptionButton from "../../../components/addOptionButton";
import BottomFloatingButton from "../../../components/bottomFloatingButton";
import OptionCard from "../../../components/option";
import { getTestById, updateTest } from "../../../utils/admin-sdk";

interface ActionsOptionsChoice {
    text: string;
    isCorrect: boolean;
}

interface ActionsOptionsAction {
    text: string;
}

interface TestDataChoice {
    _id: string;
    type: "choice";
    theme: string;
    question: string;
    answers: string[];
    answer: string;
    level: number;
}

interface TestDataAction {
    _id: string;
    type: "action";
    theme: string;
    question: string;
    answers: string[];
    answer: string;
    level: number;
}

export default function EditTest() {
    const router = useRouter();
    const id = router.query.id;

    const [testData, setTestData] = useState<TestDataChoice | TestDataAction>();

    const [testType, setTestType] = useState<"choice" | "action">("choice");
    const [themeOfTest, setThemeOfTest] = useState<string>("");
    const [question, setQuestion] = useState<string>("");
    const [actions, setActions] = useState<
        ActionsOptionsChoice[] | ActionsOptionsAction[]
    >([]);
    const [answer, setAnswer] = useState<string>("");
    const [levelOfTest, setLevelOfTest] = useState<number>();

    /* 
        <testType>: CamelCase (ChoiceTest, ActionTest)
        API Response:
        {
            <testType>: {
                _id: String
                theme: String (about the test)
                question: String
                answers: Array of Strings
                answer: String
                level: number (difficulty level from 1 to 5)
            }
        }
         
       Algorithm:
           1. Get the test with the given ID from API using SDK.
           2. Display the test data
            2.1. convert the test data to the state variables
                2.1.1 Convert CamelCase to single word (ChoiceTest -> choice, ActionTest -> action)
                2.1.2 Convert the test data to the state variables
            2.2. Display the test data
   */

    const onTestTypeChanged = (value: string) => {
        setTestType(value as "choice" | "action");
        setActions([]);
    };

    const deleteOption = (index: number) => {
        // remove object from actions list. index is the index of the object to be removed
        for (let i = 0; i < actions.length; i++) {
            if (i === index) {
                const temp = [...actions];
                temp.splice(index, 1);
                setActions(temp);
            }
        }
    };

    let apiAnswers: any = [];
    let correctAnswers: string = "";
    for (let i = 0; i < actions.length; i++) {
        apiAnswers.push(actions[i].text);
        if (testType === "choice") {
            if ((actions[i] as ActionsOptionsChoice).isCorrect) {
                correctAnswers = actions[i].text;
            }
        }
    }

    const handleUpdateButton = useCallback(async () => {
        if (testType === "choice") {
            await updateTest(
                "choice",
                {
                    ChoiceTest: {
                        // _id: testData?._id,
                        theme: themeOfTest,
                        question: question,
                        answers: apiAnswers as string[],
                        answer: correctAnswers,
                        level: levelOfTest as number,
                    },
                },
                id as string
            );
        } else {
            await updateTest(
                "action",
                {
                    ActionTest: {
                        // _id: testData?._id,
                        theme: themeOfTest,
                        question: question,
                        answers: apiAnswers as string[],
                        answer: answer,
                        level: levelOfTest as number,
                    },
                },
                id as string
            );
        }
    }, [
        testType,
        themeOfTest,
        question,
        apiAnswers,
        correctAnswers,
        levelOfTest,
        actions,
    ]);

    const prepareData = (data: any) => {
        /* 
            Response:
            testType: String (ChoiceTest, ActionTest)
            {
                <testType>: {
                    _id: {
                        $oid: String
                    }
                    theme: String (about the test)
                    question: String
                    answers: Array of Strings
                    answer: String
                    level: number (difficulty level from 1 to 5)
                }
            }
        */

        for (let key in data) {
            if (key === "ChoiceTest") {
                setTestData({
                    _id: data[key]._id.$oid,
                    type: "choice",
                    theme: data[key].theme,
                    question: data[key].question,
                    answers: data[key].answers,
                    answer: data[key].answer,
                    level: data[key].level,
                } as TestDataChoice);
            } else if (key === "ActionTest") {
                setTestData({
                    _id: data[key]._id.$oid,
                    type: "action",
                    theme: data[key].theme,
                    question: data[key].question,
                    answers: data[key].answers,
                    answer: data[key].answer,
                    level: data[key].level,
                } as TestDataAction);
            }
        }
    };

    useEffect(() => {
        getTestById(id as string).then((data) => {
            prepareData(data);
        });
    }, []);

    useEffect(() => {
        // map test data to state variables
        if (testData) {
            setTestType(testData.type);
            setThemeOfTest(testData.theme);
            setQuestion(testData.question);
            setLevelOfTest(testData.level);
            setAnswer(testData.answer);
            // map answers to actions if test type is choice set isCorrect to true if element is equal to answer
            if (testData.type === "choice") {
                let temp: ActionsOptionsChoice[] = [];
                for (let i = 0; i < testData.answers.length; i++) {
                    temp.push({
                        text: testData.answers[i],
                        isCorrect: testData.answers[i] === testData.answer,
                    });
                }
                setActions(temp);
            } else {
                let temp: ActionsOptionsAction[] = [];
                for (let i = 0; i < testData.answers.length; i++) {
                    temp.push({
                        text: testData.answers[i],
                    });
                }
                setActions(temp);
            }
        }
    }, [testData]);

    return (
        <>
            <Head>
                <title>Edit Test</title>
                <meta
                    name="description"
                    content="Generated by create next app"
                />
            </Head>
            <AbsoluteCenter w={"400px"}>
                {/* Text of question */}
                <Input
                    value={testData?._id}
                    disabled
                    border={"1px solid"}
                    borderColor={"orange.800"}
                    placeholder={"ID"}
                    focusBorderColor={"orange.500"}
                    _hover={{
                        borderColor: "orange.400",
                        bgColor: "orange.50",
                        _placeholder: { color: "blackAlpha.900" },
                    }}
                />
                <Input
                    marginTop={"10px"}
                    value={question}
                    onChange={(e) => setQuestion(e.target.value)}
                    border={"1px solid black"}
                    placeholder={"Question"}
                    focusBorderColor={"orange.500"}
                    _hover={{
                        borderColor: "orange.400",
                        bgColor: "orange.50",
                        _placeholder: { color: "blackAlpha.900" },
                    }}
                />
                <Input
                    marginTop={"10px"}
                    marginBottom={"10px"}
                    value={themeOfTest}
                    onChange={(e) => setThemeOfTest(e.target.value)}
                    border={"1px solid black"}
                    placeholder={"Theme of test"}
                    focusBorderColor={"orange.500"}
                    _hover={{
                        borderColor: "orange.400",
                        bgColor: "orange.50",
                        _placeholder: { color: "blackAlpha.900" },
                    }}
                />
                <NumberInput
                    focusBorderColor="orange.500"
                    max={5}
                    min={1}
                    value={levelOfTest}
                    onChange={(valueString) => {
                        setLevelOfTest(Number(valueString));
                    }}
                    _placeholder={{ color: "blackAlpha.900" }}
                    placeholder="Level of test"
                >
                    <NumberInputField
                        border={"1px solid black"}
                        _hover={{
                            borderColor: "orange.400",
                            bgColor: "orange.50",
                            _placeholder: { color: "blackAlpha.900" },
                        }}
                    />
                </NumberInput>
                {/* Text type switch */}
                <Center>
                    <RadioGroup
                        onChange={onTestTypeChanged}
                        value={testType}
                        colorScheme="orange"
                    >
                        <HStack justifyContent={"space-around"}>
                            <Radio value="choice">Choice</Radio>
                            <Radio value="action">Action</Radio>
                        </HStack>
                    </RadioGroup>
                </Center>
                {
                    // if emit("update_actions") is called, this will be updated
                    actions &&
                        actions.map((action, index) => {
                            return (
                                <OptionCard
                                    key={index.toString()}
                                    data={action}
                                    type={testType}
                                    index={index}
                                    onEdit={(data) => {
                                        const temp = [...actions];
                                        temp[index] = data;
                                        setActions(temp);
                                    }}
                                    onDelete={deleteOption}
                                />
                            );
                        })
                }
                <AddOptionButton
                    _type={testType}
                    onClick={() => {
                        if (testType === "choice") {
                            setActions([
                                ...actions,
                                {
                                    text: "",
                                    isCorrect: false,
                                },
                            ]);
                        } else {
                            setActions([
                                ...actions,
                                {
                                    text: "",
                                },
                            ]);
                        }
                    }}
                />
            </AbsoluteCenter>
            <BottomFloatingButton
                text="Update"
                icon={<FaUpload />}
                onClick={() => {
                    handleUpdateButton();
                }}
            />
        </>
    );
}
