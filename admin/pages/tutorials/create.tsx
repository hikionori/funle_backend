import { AbsoluteCenter, Button, Input, Text } from "@chakra-ui/react";
import Head from "next/head";
import React, { useEffect } from "react";
import LevelNodeList, {
    LevelNodeListProps,
} from "../../components/levelNodeList";
import AddOptionButton from "../../components/addOptionButton";
import useTutorialStore from "../../utils/states/tutorial";
import BottomFloatingButton from "../../components/bottomFloatingButton";
import { FaPlus } from "react-icons/fa";
import { useRouter } from "next/router";

export default function CreateNewTutorial() {

    const router = useRouter();

    const {title, setTitle} = useTutorialStore((state: any) => ({title: state.title, setTitle: state.setTitle}));
    const {theme, setTheme} = useTutorialStore((state: any) => ({theme: state.theme, setTheme: state.setTheme}));
    const {contentLevels, setContentLevels} = useTutorialStore((state: any) => ({contentLevels: state.contentLevels, setContentLevels: state.setContentLevels}));

    const {addLevel} = useTutorialStore((state: any) => ({addLevel: state.addLevel}));
    const {createTutor} = useTutorialStore((state: any) => ({createTutor: state.createTutor}));

    return (
        <>
            <Head>
                <title>Create new tutorial</title>
            </Head>
            <main>
                <AbsoluteCenter>
                    <Input
                        value={title}
                        onChange={(e) => setTitle(e.target.value)}
                        border={"1px solid black"}
                        placeholder={"Title"}
                        focusBorderColor="orange.500"
                        _hover={{
                            borderColor: "orange.400",
                            bgColor: "orange.50",
                            _placeholder: { color: "blackAlpha.900" },
                        }}
                    />
                    <Input
                        marginTop={"10px"}
                        marginBottom={"10px"}
                        value={theme}
                        onChange={(e) => setTheme(e.target.value)}
                        border={"1px solid black"}
                        placeholder={"Theme"}
                        focusBorderColor="orange.500"
                        _hover={{
                            borderColor: "orange.400",
                            bgColor: "orange.50",
                            _placeholder: { color: "blackAlpha.900" },
                        }}
                    />

                    <LevelNodeList />

                    <AddOptionButton
                        onClick={() => {
                            addLevel();
                        }}
                    />
                </AbsoluteCenter>
                <BottomFloatingButton 
                    text="Create"
                    icon={<FaPlus />}
                    onClick={async () => {
                        createTutor();
                        router.push("/tutorials");
                    }}
                />
            </main>
        </>
    );
}
