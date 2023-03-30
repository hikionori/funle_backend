import React from "react";
import Head from "next/head";
import BottomFloatingButton from "../../components/bottomFloatingButton";
import { useRouter } from "next/router";
import { FaPlus } from "react-icons/fa";
import { TutorialCardProps } from "../../components/card/tutorialCard";
import { deleteInfo, getAllInfos, Info } from "../../utils/admin-sdk/info";
import { AbsoluteCenter, Box, Spinner } from "@chakra-ui/react";
import TutorialCardList from "../../components/cardList/tutorialCardList";

interface ThemeCardListProps {
    theme: string;
    cards: TutorialCardProps[];
}

export default function Tutorilas() {
    const router = useRouter();
    const [ready, setReady] = React.useState(false);

    const [cardLists, setCardLists] = React.useState<ThemeCardListProps[]>([]);
    const themeList: string[] = [];

    const editButtonHandler = (id: string) => {
        router.push("/tests/edit/" + id);
    };

    const deleteButtonHandler = async (id: string) => {
        await deleteInfo(id);
        setReady(false);
    };

    const prepareData = (data: Info[]) => {
        /* 
            API Response:
            [
                {
                    "_id": {
                        "$oid": "60a1b1f1b9d4b8b5b0d1b1f1"
                    },
                    "title": "Test",
                    "theme": "Test",
                    "content_levels": [
                        [
                            0,
                            [{
                                "content_type": "text", // text, image
                                "data": "Test" // text, image base64
                            }]
                        ],
                        [
                            1,
                            [{
                                "content_type": "text", // text, image
                                "data": "Test" // text, image base64
                            },
                            {
                                "content_type": "text", // text, image
                                "data": "Test" // text, image base64
                            }]
                        ]
                    ]
                },
                {
                    "_id": {
                        "$oid": "60a1b1f1b9d4b8b5b0d1b1f1"
                    },
                    "title": "Test",
                    "theme": "Test",
                    "content_levels": [
                        [
                            0,
                            [{
                                "content_type": "text", // text, image
                                "data": "Test" // text, image base64
                            }]
                        ]
                    ]
                }

            ]

            Task:
            1. Create a list of themes
            2. Create a list of cards for each theme
            3. Create a list of cardLists
        */
        for (let i = 0; i < data.length; i++) {
            const theme = data[i].theme;
            if (!themeList.includes(theme)) {
                themeList.push(theme);
            }
        }

        const cardLists: ThemeCardListProps[] = [];
        for (let i = 0; i < themeList.length; i++) {
            const theme = themeList[i];
            const cards: TutorialCardProps[] = [];
            for (let j = 0; j < data.length; j++) {
                const info = data[j];
                if (info.theme === theme) {
                    cards.push({
                        title: info.title,
                        id: info._id["$oid"],
                        onClick: editButtonHandler,
                        onDelete: deleteButtonHandler,
                    });
                }
            }
            cardLists.push({
                theme: theme,
                cards: cards,
            });
        }

        setCardLists(cardLists);

        // remove duplicates
        setCardLists((prev) => {
            const unique = prev.filter(
                (v, i, a) => a.findIndex((t) => t.theme === v.theme) === i
            );
            return unique;
        });
    };

    React.useEffect(() => {
        if (!ready) {
            getAllInfos().then((data) => {
                prepareData(data);
                setReady(true);
            });
        }
    }, [ready]);

    return (
        <>
            <Head>
                <title>Tutorilas</title>
            </Head>
            <Box>
                {ready ? (
                    cardLists.map((cardList) => (
                        <TutorialCardList
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
                <BottomFloatingButton
                    icon={<FaPlus size={30} />}
                    onClick={() => {
                        router.push("/tutorials/create");
                    }}
                />
            </Box>
        </>
    );
}
