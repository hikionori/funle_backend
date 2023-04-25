import { Box, Button, Center, Flex, Text } from "@chakra-ui/react";
import { Content } from "../../utils/admin-sdk";
import LevelNode, { LevelNodeProps } from "../levelNode";
import { useEffect, useState } from "react";
import { NodeProps } from "../node";
import { FaPlus, FaTrash } from "react-icons/fa";

import useTutorialStore from "../../utils/states/tutorial";


export interface LevelNodeListProps {
    nodeLevels: LevelNodeProps[];
    // add Symbol.iterator to LevelNodeListProps
    [Symbol.iterator]?: () => Iterator<LevelNodeProps>;
}

// List of levelNode component
export default function LevelNodeList(
    props: LevelNodeListProps
) {
    const {contentLevels} = useTutorialStore((state: any) => ({contentLevels: state.contentLevels}));
    const {deleteLevel} = useTutorialStore((state: any) => ({deleteLevel: state.deleteLevel}));
    const {addNode, editNode, deleteNode} = useTutorialStore((state: any) => ({addNode: state.addNode, editNode: state.editNode, deleteNode: state.deleteNode}));


    useEffect(() => {
        console.log(contentLevels);
        // reload page
    })

    
    return (
        <Flex flexDirection={"column"}>
            {/* sort by first element in level and then display */}
            {contentLevels &&
                contentLevels
                    .sort((a: any, b: any) => a[0] - b[0])
                    .map((level: any, index: any) => {
                        return (
                            <Flex>
                                <LevelNode
                                    key={index}
                                    index={level[0]}
                                    nodes={level[1]}
                                    editHandler={editNode}
                                    deleteHandler={deleteNode}
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
                    })}
        </Flex>
    );
}
