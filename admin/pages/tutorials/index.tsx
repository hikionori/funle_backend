import React from "react";
import Head from "next/head";
import BottomFloatingButton from "../../components/bottomFloatingButton";
import router from "next/router";
import { FaPlus } from "react-icons/fa";

export default function Tutorilas() {
    return (
        <>
            <Head>
                <title>Tutorilas</title>
            </Head>
            <main>
                <h1>Tutorilas</h1>
                <BottomFloatingButton
                    icon={<FaPlus />}
                    onClick={() => {
                        router.push("/tutorials/create");
                    }}
                />
            </main>
        </>
    );
}
