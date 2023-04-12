import { Box, Flex, Text } from "@chakra-ui/react";
import { Content } from "../../utils/admin-sdk";
import Image from "next/image";

import Node, { NodeProps } from "../node/index";

export interface LevelNodeProps {
    nodes: NodeProps[];
}

// add Symbol.iterator to LevelNodeProps

// Level of node component
export default function LevelNode(props: LevelNodeProps) {
    const { nodes } = props;

    const nodeOnEdit = (index: number) => {
        console.log("Edit node at index: " + index);
    };

    const nodeOnDelete = (index: number) => {
        console.log("Delete node at index: " + index);
    };

    return (
        <Box w={"100%"}>
            <Flex direction={"row"}>
                {nodes.map((node, index) => {
                    return (
                        <Node
                            key={index}
                            index={index}
                            content_type={node.content_type}
                            data={node.data}
                            onEdit={nodeOnEdit}
                            onDelete={nodeOnDelete}
                        />
                    );
                })}
            </Flex>
        </Box>
    );
}
