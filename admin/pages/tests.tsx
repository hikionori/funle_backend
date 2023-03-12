import React, { useEffect } from "react";
import Head from "next/head";
import { AbsoluteCenter, Box, Button, Center, Spinner } from "@chakra-ui/react";
import TestCard, { TestCardProps } from "../components/card";
import CardList from "../components/cardList";
import { FaPlus, FaPlusCircle } from "react-icons/fa";
import { color } from "framer-motion";

interface ThemeCardListProps {
    theme: string;
    cards: TestCardProps[];
}

export default function Tests() {
    // Algorithm to sort cards into themes
    // 1. Get all tests from SDK
    // 2. For each test, get the theme and collect to a unique list
    // 3. For each theme, create a new CardList with the tests that match the theme and add edit and delete buttons
    // 4. Render the CardLists
    // List of themes structure:
    /*  [
        {
            theme: "test",
            cards: [
                {    
                    id: "1",
                    text: "test",
                    type: "choice",
                    onClick: () => {},
                    onDelete: () => {},
                  },
                 {
                    id: "2",
                    text: "test",
                    type: "action",
                    onClick: () => {},
                    onDelete: () => {},
                  },],
        },
        {
            theme: "test2",
            cards: [
                {
                    id: "3",
                    text: "test",
                    type: "choice",
                    onClick: () => {},
                    onDelete: () => {},
                },
                {
                    id: "4",
                    text: "test",
                    type: "action",
                    onClick: () => {},
                    onDelete: () => {},
                },
            ],
        },
     ]
    */
    
    const [ready, setReady] = React.useState(false);
    
    const [themeCardLists, setThemeCardLists] = React.useState<
        ThemeCardListProps[]
    >([]);
    
    const themeList: string[] = [];


    useEffect(() => {
        // TODO: Get tests from SDK
        setThemeCardLists([
            {
                theme: "test",
                cards: [
                    {
                        id: "1",
                        text: "test",
                        type: "choice",
                        onClick: () => {
                            console.log("test");
                        },
                        onDelete: () => {
                            console.log("test");
                        },
                    },
                    {
                        id: "2",
                        text: "test",
                        type: "action",
                        onClick: () => {
                            console.log("test");
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
                        onClick: () => {
                            console.log("test");
                        },
                        onDelete: () => {
                            console.log("test");
                        },
                    },
                    {
                        id: "4",
                        text: "test",
                        type: "action",
                        onClick: () => {
                            console.log("test");
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
        }
        , 1000);
    }, [ready]);

    const editButtonHandler = (id: string) => {
        console.log("edit button clicked at card with id: " + id);
    };

    const deleteButtonHandler = (id: string) => {
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
                    themeCardLists.map((themeCardList) => (
                        <CardList
                            theme={themeCardList.theme}
                            cards={themeCardList.cards}
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
            {/* Create FloatingBottomButton */}
            {/* TODO: onClick for create new test */}
            <Button
                position={"fixed"}
                bgColor={"white"}
                _hover={{
                    bgColor: "black",
                    color: "white",
                    transition: "50ms linear",
                }}
                borderRadius="full"
                bottom="40px"
                right={"40px"}
                height="70px"
                width="70px"
                zIndex={2}
                onClick={() => {
                    console.log("create new test");
                }}
            >
                <FaPlus size={"30px"} />
            </Button>
        </>
    );
}
