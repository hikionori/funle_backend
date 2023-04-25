import { Box, Flex, Text } from "@chakra-ui/react";
import { Content } from "../../utils/admin-sdk";
import Image from "next/image";

import Node, { NodeProps } from "../node/index";
import { useEffect, useState } from "react";

export interface LevelNodeProps {
    index: number;
    nodes: NodeProps[];

    deleteHandler: Function;
    editHandler: Function;
}

// add Symbol.iterator to LevelNodeProps

// Level of node component
export default function LevelNode(props: LevelNodeProps) {
    const [nodes, setNodes] = useState<NodeProps[]>([]);
    const editHandlerP = props.editHandler; // is setState function
    const deleteHandlerP = props.deleteHandler; // is setState function
    const levelIndex = props.index;

    useEffect(() => {
        setNodes(props.nodes);
    }, []);

    return (
        <Box w={"100%"}>
            <Flex direction={"row"}>
                {props.nodes && props.nodes.map((node, index) => {
                    return (
                        <Node
                            key={index}
                            index={index}
                            content_type={node.content_type}
                            data={node.data}
                            levelIndex={levelIndex}
                            deleteHandler={() => {
                                deleteHandlerP(levelIndex, node.index);
                            }}
                            editHandler={() => {
                                editHandlerP(levelIndex, node.index);
                            }}
                        />
                    );
                })}
            </Flex>
        </Box>
    );
}
