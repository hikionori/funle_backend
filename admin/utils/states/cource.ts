import create from "zustand";

/*
    JSON:
    {
        _id: {
            $oid: string
        },
        title: string,
        description: string,
        levels: [
            [
                level index: number,
                [ array of nodes
                    { node
                        id: string,
                        title: string,
                        mini_image: string,
                        type_ : string, // info | test
                        n_of_tests: number | string, // if type_ == "test" else "None"
                    }
                ]
            ]
        ]
    }
*/


export const useCourseStore = create((set, get: any) => ({
    _id: "",
    title: "",
    description: "",
    levels: [],

    // setters
    setCourseId: (id: string) => set({ _id: id }),
    setTitle: (title: string) => set({ title }),
    setDescription: (description: string) => set({ description }),
    setLevels: (levels: any[]) => set({ levels }),
    
}));