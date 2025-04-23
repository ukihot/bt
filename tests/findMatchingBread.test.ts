import { BreadType } from "@/app/bt.types";
import { findMatchingBread } from "@/app/utils/findMatchingBread";

describe("findMatchingBread", () => {
    it("should return undefined if input is undefined", () => {
        expect(findMatchingBread(undefined)).toBeUndefined();
    });

    it("should return undefined if no matching bread is found", () => {
        expect(findMatchingBread("xyz")).toBeUndefined();
    });

    it("should return the correct BreadType for a valid input", () => {
        expect(findMatchingBread("anp")).toBe(BreadType.Anpan);
        expect(findMatchingBread("a")).toBe(BreadType.Anpan);
        expect(findMatchingBread("na")).toBe(BreadType.Naan);
    });
});
