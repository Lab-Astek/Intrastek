import Bar from "./bar";

type PageProps = {
    children?: React.ReactNode;
    title: string;
}

export default function Page({ children, title }: PageProps) {
    return <div>
        <Bar title={title} />
        <main className="flex min-h-screen flex-col items-center p-24">
            {children}
        </main>
    </div>
}