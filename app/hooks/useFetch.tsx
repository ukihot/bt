import { useEffect, useState } from "react";

export function useFetch<T>(url: string) {
    const [data, setData] = useState<T | null>(null);
    const [error, setError] = useState<string | null>(null);
    useEffect(() => {
        let cancelled = false;
        fetch(url)
            .then((res) => {
                if (!res.ok) throw new Error(`Status ${res.status}`);
                return res.json() as Promise<T>;
            })
            .then((json) => !cancelled && setData(json))
            .catch((err) => !cancelled && setError(err.message));
        return () => {
            cancelled = true;
        };
    }, [url]);
    return { data, error };
}
