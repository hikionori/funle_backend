import { AbsoluteCenter, Button, Input, Text } from "@chakra-ui/react";
import Head from "next/head";
import React, { useEffect } from "react";
import LevelNodeList, {
    LevelNodeListProps,
} from "../../../components/levelNodeList";
import AddOptionButton from "../../../components/addOptionButton";
import useTutorialStore from "../../../utils/states/tutorial";
import BottomFloatingButton from "../../../components/bottomFloatingButton";
import { FaPen, FaPlus, FaUpload } from "react-icons/fa";
import { useRouter } from "next/router";

export default function EditTutorial() {

    const router = useRouter();
    const id = router.query.id;

    const {_id, setTutorialId} = useTutorialStore((state: any) => ({_id: state._id, setTutorialId: state.setTutorialId}));
    const {title, setTitle} = useTutorialStore((state: any) => ({title: state.title, setTitle: state.setTitle}));
    const {theme, setTheme} = useTutorialStore((state: any) => ({theme: state.theme, setTheme: state.setTheme}));
    const {contentLevels, setContentLevels} = useTutorialStore((state: any) => ({contentLevels: state.contentLevels, setContentLevels: state.setContentLevels}));

    const {getTutor, addLevel, editTutorial} = useTutorialStore((state: any) => ({getTutor: state.getTutor, addLevel: state.addLevel, editTutorial: state.editTutorial}));

    useEffect(() => {
        setTutorialId(id?.toString());
        getTutor(id);
    }, [])

    return (
        <>
         <Head>
                <title>Edit tutorial</title>
            </Head>
            <main>
                <AbsoluteCenter>
                    <Input
                        value={_id}
                        disabled
                        border={"1px solid black"}
                        placeholder={"ID"}
                        _hover={{
                            borderColor: "orange.400",
                            bgColor: "orange.50",
                            _placeholder: { color: "blackAlpha.900" },
                        }}
                    />
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
                    text="Update"
                    icon={<FaPen />}
                    onClick={async () => {
                        await editTutorial();
                        router.push("/tutorials");
                    }}
                />
            </main>
        </>
    )
}