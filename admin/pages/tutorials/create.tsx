import { AbsoluteCenter, Text } from "@chakra-ui/react";
import Head from "next/head";
import React from "react";

export default function CreateNewTutorial() {
    return (
        <>
            <Head>
                <title>Create new tutorial</title>
            </Head>
            <main>
                <AbsoluteCenter>
                    <Text fontSize="5xl">Create new tutorial</Text>
                </AbsoluteCenter>
            </main>
        </>
    );
}
