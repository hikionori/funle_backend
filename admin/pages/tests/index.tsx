import React, { useEffect } from "react";
import Head from "next/head";
import { AbsoluteCenter, Box, Button, Center, Spinner } from "@chakra-ui/react";
import TestCard, { TestCardProps } from "../../components/card";
import CardList from "../../components/cardList";
import { FaPlus, FaPlusCircle } from "react-icons/fa";
import { color } from "framer-motion";
import { useRouter } from "next/router";
import BottomFloatingButton from "../../components/bottomFloatingButton";

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
                    id: string,
                    theme: string,
                    question: string,
                    answers: string[],
                    answer: string,
                    level: number,
                },
                ...
            ]
            tests_with_acting: [
                {
                    id: string,
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
    */
    //! Function in progress
    const prepareData = (data: any) => {
        for (let i = 0; i < data.tests.length; i++) {
            const theme = data.tests[i].theme;
            if (!themeList.includes(theme)) {
                themeList.push(theme);
            }
        }
        for (let i = 0; i < data.tests_with_acting.length; i++) {
            const theme = data.tests_with_acting[i].theme;
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
                        id: data.tests[j].id,
                        text: data.tests[j].question,
                        type: "choice",
                        onClick: (id: string) => {
                            editButtonHandler(id);
                        },
                        onDelete: () => {
                            deleteButtonHandler(data.tests[j].id);
                        },
                    });
                }
            }
            for (let j = 0; j < data.tests_with_acting.length; j++) {
                if (data.tests_with_acting[j].theme === theme) {
                    cards.push({
                        id: data.tests_with_acting[j].id,
                        text: data.tests_with_acting[j].question,
                        type: "action",
                        onClick: (id: string) => {
                            editButtonHandler(id);
                        },
                        onDelete: () => {
                            deleteButtonHandler(data.tests_with_acting[j].id);
                        },
                    });
                }
            }
            cardLists.push({
                theme: theme,
                cards: cards,
            });
        }
    };

    useEffect(() => {
        // TODO: Get tests from SDK
        setCardLists([
            {
                theme: "test",
                cards: [
                    {
                        id: "1",
                        text: "test",
                        type: "choice",
                        onClick: (id: string) => {
                            editButtonHandler(id);
                        },
                        onDelete: () => {
                            // TODO: Delete test from SDK and update list
                            console.log("test");
                        },
                    },
                    {
                        id: "2",
                        text: "test",
                        type: "action",
                        onClick: (id: string) => {
                            editButtonHandler(id);
                        },
                        onDelete: () => {
                            console.log("test");
                        },
                    },
                ],
            },
            {
                theme: "test2",
                cards: [
                    {
                        id: "3",
                        text: "test",
                        type: "choice",
                        onClick: (id: string) => {
                            editButtonHandler(id);
                        },
                        onDelete: () => {
                            console.log("test");
                        },
                    },
                    {
                        id: "4",
                        text: "test",
                        type: "action",
                        onClick: (id: string) => {
                            editButtonHandler(id);
                        },
                        onDelete: () => {
                            console.log("test");
                        },
                    },
                ],
            },
        ]);
        setTimeout(() => {
            setReady(true);
        }, 200);
    }, [ready]);

    const editButtonHandler = (id: string) => {
        router.push("/tests/edit/" + id);
    };

    const deleteButtonHandler = (id: string) => {
        // Request to delete test from SDK and update list
        console.log("delete button clicked at card with id: " + id);
    };

    return (
        <>
            <Head>
                <title>tests</title>
                <meta
                    name="description"
                    content="Generated by create next app"
                />
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
