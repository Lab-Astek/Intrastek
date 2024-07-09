"use client";
import { roboto } from "@/theme";
import { createTheme, PaletteMode, ThemeProvider } from "@mui/material";
import { createContext, useMemo, useState } from "react";

export const MUIWrapperContext = createContext({
    toggleColorMode: () => { },
});

export default function MUIWrapper({
    children,
}: {
    children: React.ReactNode;
}) {
    const [mode, setMode] = useState<PaletteMode>("light");
    const muiWrapperUtils = useMemo(
        () => ({
            toggleColorMode: () => {
                setMode((prevMode) => (prevMode === "light" ? "dark" : "light"));
            },
        }),
        []
    );

    const theme = useMemo(
        () =>
            createTheme({
                typography: {
                    fontFamily: roboto.style.fontFamily,
                },
                palette: {
                    mode,
                },
            }),
        [mode]
    );

    return (
        <MUIWrapperContext.Provider value={muiWrapperUtils}>
            <ThemeProvider theme={theme}>{children}</ThemeProvider>
        </MUIWrapperContext.Provider>
    );
}