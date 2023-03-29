import React, { useEffect } from "react";
import Head from "next/head";
import { AbsoluteCenter, Box, Button, Center, Spinner } from "@chakra-ui/react";
import { TestCardProps } from "../../components/card/testCard";
import TestCardList from "../../components/cardList/testCardList";
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
        if (!ready) {
            getAllTests().then((data) => {
                prepareData(data);
                setReady(true);
            });
        }
    }, [ready]);

    const editButtonHandler = (id: string) => {
        router.push("/tests/edit/" + id);
    };

    const deleteButtonHandler = async (id: string, test_type: string) => {
        // Request to delete test from SDK and update list
        await deleteTest(id, test_type as "choice" | "action");
        // Update list
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
                        <TestCardList
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
