import { useState, useEffect } from "react";

export function useNameCheck(name: string) {
    const [isUnique, setIsUnique] = useState<boolean | null>(null);
    const [checking, setChecking] = useState(false);

    useEffect(() => {
        const trimmed = name.trim();
        if (trimmed.length < 3 || trimmed.length > 10) {
            setIsUnique(null);
            setChecking(false);
            return;
        }

        setChecking(true);
        let cancelled = false;
        fetch("/api/ranking/check-name", {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ name: trimmed }),
        })
            .then((res) => res.json())
            .then((data) => {
                if (!cancelled) setIsUnique(Boolean(data.isUnique));
            })
            .catch(() => !cancelled && setIsUnique(false))
            .finally(() => !cancelled && setChecking(false));

        return () => {
            cancelled = true;
        };
    }, [name]);

    return { isUnique, checking };
}
