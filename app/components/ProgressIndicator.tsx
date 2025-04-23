interface ProgressIndicatorProps {
    progress: number;
    isHealthy: boolean;
}

export const ProgressIndicator = ({
    progress,
    isHealthy,
}: ProgressIndicatorProps) => {
    return (
        <div className="progress-indicator segmented">
            <span
                className={`progress-indicator-bar ${!isHealthy ? "!bg-rose-500" : ""}`}
                style={{ width: `${progress}%` }}
            />
        </div>
    );
};
