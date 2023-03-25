import React, { useEffect } from "react";
import Head from "next/head";
import { AbsoluteCenter, Box, Button, Center, Spinner } from "@chakra-ui/react";
import TestCard, { TestCardProps } from "../../components/card";
import CardList from "../../components/cardList";
import { FaPlus, FaPlusCircle } from "react-icons/fa";
import { useRouter } from "next/router";
import BottomFloatingButton from "../../components/bottomFloatingButton";

import { AllTests, deleteTest, getAllTests } from "../../utils/admin-sdk";

interface ThemeCardListProps {
    theme: string;
    cards: TestCardProps[];
}

export default function Tests() {
    const router = useRouter();
    const [ready, setReady] = React.useState(false);

    const [cardLists, setCardLists] = React.useState<ThemeCardListProps[]>([]);

    const themeList: string[] = [];

    /* 
        API response for get all tests:
        {
            tests: [
                {
                    id: {"$oid": string},
                    theme: string,
                    question: string,
                    answers: string[],
                    answer: string,
                    level: number,
                },
                ...
            ]
            tests_with_actions: [
                {
                    id: {"$oid": string},
                    theme: string,
                    question: string,
                    answers: string[],
                    answer: string,
                    level: number,
                },
                ...
            ]
        }

        Algorithm:
        1. Get all tests from SDK
        2. For each test, get the theme and collect to a unique list
        3. For each theme, create a new CardList with the tests that match the theme 
            and add edit and delete buttons, as type "choice" is tests and "action" is tests_with_acting in the API response
        4. Render the CardLists

        As id set "$oid" in _id
    */
    const prepareData = (data: AllTests) => {
        for (let i = 0; i < data.tests.length; i++) {
            const theme = data.tests[i].theme;
            if (!themeList.includes(theme)) {
                themeList.push(theme);
            }
        }
        for (let i = 0; i < data.tests_with_actions.length; i++) {
            const theme = data.tests_with_actions[i].theme;
            if (!themeList.includes(theme)) {
                themeList.push(theme);
            }
        }

        const cardLists: ThemeCardListProps[] = [];
        for (let i = 0; i < themeList.length; i++) {
            const theme = themeList[i];
            const cards: TestCardProps[] = [];
            for (let j = 0; j < data.tests.length; j++) {
                if (data.tests[j].theme === theme) {
                    cards.push({
                        id: data.tests[j]._id["$oid"],
                        text: data.tests[j].question,
                        type: "choice",
                        onClick: (id: string) => {
                            editButtonHandler(id);
                        },
                        onDelete: () => {
                            let id = data.tests[j]._id["$oid"];
                            deleteButtonHandler(id, "choice");
                        },
                    });
                }
            }
            for (let j = 0; j < data.tests_with_actions.length; j++) {
                if (data.tests_with_actions[j].theme === theme) {
                    cards.push({
                        // id: get data from data.tests_with_actions[j].id.$oid
                        id: data.tests_with_actions[j]._id["$oid"],
                        text: data.tests_with_actions[j].question,
                        type: "action",
                        onClick: (id: string) => {
                            editButtonHandler(id);
                        },
                        onDelete: () => {
                            deleteButtonHandler(data.tests_with_actions[j]._id["$oid"], "action");
                        },
                    });
                }
            }
            setCardLists((prev) => [
                ...prev,
                {
                    theme: theme,
                    cards: cards,
                },
            ]);

            // remove duplicates
            setCardLists((prev) => {
                const unique = prev.filter(
                    (v, i, a) => a.findIndex((t) => t.theme === v.theme) === i
                );
                return unique;
            });

        }
    };

    useEffect(() => {
        setCardLists([]);
        getAllTests().then((data) => {
            prepareData(data);
        });
        setReady(true);
    }, [ready]);

    const editButtonHandler = (id: string) => {
        router.push("/tests/edit/" + id);
    };

    const deleteButtonHandler = async (id: string, test_type: string) => {
        // Request to delete test from SDK and update list
        await deleteTest(id, test_type as "choice" | "action");
        setReady(false);
    };

    return (
        <>
            <Head>
                <title>Tests</title>
            </Head>
            <Box>
                {ready ? (
                    cardLists.map((cardList) => (
                        <CardList
                            theme={cardList.theme}
                            cards={cardList.cards}
                        />
                    ))
                ) : (
                    <AbsoluteCenter>
                        <Spinner
                            thickness="3px"
                            speed="0.65s"
                            emptyColor="gray.200"
                            color="orange.400"
                            size="xl"
                        />
                    </AbsoluteCenter>
                )}
            </Box>
            <BottomFloatingButton
                onClick={() => {
                    router.push("/tests/create");
                }}
                icon={<FaPlus size={30} />}
            />
        </>
    );
}
