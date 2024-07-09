import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";
import { AppRouterCacheProvider } from "@mui/material-nextjs/v13-appRouter";
import CssBaseline from "@mui/material/CssBaseline";
import MUIWrapper from "@/components/muiWrapper";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "Intrastek",
  description: "Intranet for Asteks",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={inter.className}>
        <AppRouterCacheProvider>
          <MUIWrapper>
            <CssBaseline />
            {children}
          </MUIWrapper>
        </AppRouterCacheProvider>
      </body>
    </html>
  );
}
