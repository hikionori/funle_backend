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
import { FaPlus } from "react-icons/fa";
import AddOptionButton from "../../components/addOptionButton";
import BottomFloatingButton from "../../components/bottomFloatingButton";
import OptionCard from "../../components/option";
import {
    ActionTest,
    ActionTestBuilder,
    ChoiceTestBuilder,
    createTest,
    Test,
    TestType,
} from "../../utils/admin-sdk";

interface ActionsOptionsChoice {
    text: string;
    isCorrect: boolean;
}

interface ActionsOptionsAction {
    text: string;
}

export default function CreateNewTest() {
    
    const router = useRouter();
    
    const [testType, setTestType] = useState<"choice" | "action">("choice"); // "choice" or "action"

    const [actions, setActions] = useState<
        ActionsOptionsChoice[] | ActionsOptionsAction[]
    >([]);
    const [question, setQuestion] = useState<string>("");
    const [themeOfTest, setThemeOfTest] = useState<string>("");
    const [levelOfTest, setLevelOfTest] = useState<number>();

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

    // wrap the function in useCallback to prevent infinite loop
    const handleCreateTestButton = useCallback(async () => {
        if (testType === "choice") {
            let apiObject = new ChoiceTestBuilder();
            apiObject.setId("None");
            apiObject.setTheme(themeOfTest);
            apiObject.setQuestion(question);
            apiObject.setAnswers(apiAnswers as string[]);
            apiObject.setAnswer(correctAnswers);
            apiObject.setLevel(levelOfTest as number);

            createTest("choice", {
                "ChoiceTest": {
                    "theme": themeOfTest,
                    "question": question,
                    "answers": apiAnswers as string[],
                    "answer": correctAnswers,
                    "level": levelOfTest as number,
                }
            });
            router.back();
        } else {
            let apiObject = new ActionTestBuilder();
            apiObject.setId("None");
            apiObject.setTheme(themeOfTest);
            apiObject.setQuestion(question);
            apiObject.setAnswers(apiAnswers as string[]);
            apiObject.setAnswer(actions[actions.length - 1].text);
            apiObject.setLevel(levelOfTest as number);
            let json = apiObject.toJSON();
            console.log(json);
            createTest("action", {
                "ActionTest": {
                    "theme": themeOfTest,
                    "question": question,
                    "answers": apiAnswers as string[],
                    "answer": actions[actions.length - 1].text,
                    "level": levelOfTest as number
                }
            });
            router.back();
        }
    }, [actions, question, themeOfTest, levelOfTest, testType]);

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

    return (
        <>
            <Head>
                <title>Create New Test</title>
            </Head>
            <AbsoluteCenter w={"400px"}>
                {/* Text of question */}
                <Input
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
                <NumberInput focusBorderColor="orange.500">
                    <NumberInputField
                        placeholder="Level of question"
                        value={levelOfTest}
                        onChange={(e) => setLevelOfTest(Number(e.target.value))}
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
                text="Create"
                icon={<FaPlus />}
                onClick={async () => {
                    await handleCreateTestButton();
                }}
            />
        </>
    );
}
