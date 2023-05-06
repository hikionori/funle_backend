import { AbsoluteCenter, Input, Text, Textarea } from "@chakra-ui/react";
import { useRouter } from "next/router";
import { useCourseStore } from "../../../utils/states/cource";
import { useEffect } from "react";
import Head from "next/head";
import CourceLevelList from "../../../components/levelNodeList/courseList";
import AddOptionButton from "../../../components/addOptionButton";
import BottomFloatingButton from "../../../components/bottomFloatingButton";
import { FaPen } from "react-icons/fa";

export default function EditCource() {
    const router = useRouter();
    const id = router.query.id;

    const { _id, setCourseId } = useCourseStore((state: any) => ({
        _id: state._id,
        setCourseId: state.setCourseId,
    }));
    const { title, setTitle } = useCourseStore((state: any) => ({
        title: state.title,
        setTitle: state.setTitle,
    }));
    const { description, setDescription } = useCourseStore((state: any) => ({
        description: state.description,
        setDescription: state.setDescription,
    }));
    const { levels, setLevels } = useCourseStore((state: any) => ({
        levels: state.levels,
        setLevels: state.setLevels,
    }));

    const { getCourse, addLevel, editCourse } = useCourseStore(
        (state: any) => ({
            getCourse: state.getCourse,
            addLevel: state.addLevel,
            editCourse: state.editCourse,
        })
    );

    useEffect(() => {
        setCourseId(id?.toString());
        getCourse(id?.toString());
    }, [id]);

    return (
        <>
            <Head>
                <title>Edit cource</title>
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
                        marginY={"5px"}
                    />
                    <Textarea
                        value={description}
                        onChange={(e) => setDescription(e.target.value)}
                        border={"1px solid black"}
                        placeholder={"Description"}
                        focusBorderColor="orange.500"
                        _hover={{
                            borderColor: "orange.400",
                            bgColor: "orange.50",
                            _placeholder: { color: "blackAlpha.900" },
                        }}
                    />
                    <CourceLevelList />
                    <AddOptionButton onClick={() => addLevel()} />
                </AbsoluteCenter>
                <BottomFloatingButton
                    icon={<FaPen />}
                    onClick={() => {editCourse(); router.push("/cources")}}
                    text="Update"
                />
            </main>
        </>
    );
}
