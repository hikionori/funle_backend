import { Flex, Center, Button } from "@chakra-ui/react";
import { FaTrash, FaPlus } from "react-icons/fa";
import { useCourseStore } from "../../utils/states/cource";
import CourceLevel from "../levelNode/courseLevel";




export default function CourceLevelList() {
    const {levels} = useCourseStore((state: any) => ({levels: state.levels}));
    const {deleteLevel} = useCourseStore((state: any) => ({deleteLevel: state.deleteLevel}));
    const {addNode, editNode, deleteNode} = useCourseStore((state: any) => ({addNode: state.addNode, editNode: state.editNode, deleteNode: state.deleteNode}));

    return (
        <Flex flexDirection={"column"}>
            {levels &&
                levels.sort((a: any, b: any) => a[0] - b[0])
                .map((level: any, index: any) => {
                    return (
                        <Flex>
                            <CourceLevel
                                key={index}
                                index={level[0]}
                                nodes={level[1]}
                                deleteHandler={deleteNode}
                                editHandler={editNode}
                            />
                            <Center flexDirection={"column"}>
                                <Button
                                    onClick={
                                        () => {
                                            deleteLevel(level[0])
                                        }
                                    }
                                    _hover={{
                                        color: "orange.500",
                                    }}
                                >
                                    <FaTrash />
                                </Button>
                                <Button
                                    onClick={
                                        () => {
                                            addNode(level[0])
                                        }}
                                    _hover={{
                                        color: "orange.500",
                                    }}
                                >
                                    <FaPlus />
                                </Button>
                            </Center>
                        </Flex>
                    );
                })
            }
        </Flex>
    )
}