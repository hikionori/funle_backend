import Head from "next/head";
import Image from "next/image";
// import { Inter } from "@next/font/google";
import styles from "../styles/Home.module.css";

import Navbar from "../components/sidebar";
import SimpleSidebar from "../components/sidebar";
import Cources from "./cources";
import Tutorilas from "./tutorials";
import { AbsoluteCenter, Box, Highlight, Text } from "@chakra-ui/react";

// const inter = Inter({ subsets: ["latin"] });

export default function Home() {
    return (
        <>
            <Head>
                <title>Home page of admin CMS</title>
            </Head>
            <Box>
                <AbsoluteCenter>
                    <Highlight query={"Funle"} styles={{ px: '4', py: '2', rounded: 'full', bg: 'orange.200'}}>Це панель адміністратора даних або CMS мобільного застосунку FunLe</Highlight>
                    <Box boxSize={"20px"}/>
                    <Text>Для роботи з даними перейдіть до певного вікна за допомогою меню зліва</Text>
                </AbsoluteCenter>
            </Box>
        </>
    );
}
