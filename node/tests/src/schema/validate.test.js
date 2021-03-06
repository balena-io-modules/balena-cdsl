const jels = require('jellyschema');

test('validate does not throw if the input is invalid', () => {
    js = new jels.JellySchema({
        type: "string",
        pattern: "^[0-9]+$"
    });

    expect(js.validate(undefined)).toBe(false);
})

test('string validation', () => {
    js = new jels.JellySchema({
        type: "string",
        pattern: "^[0-9]+$",
        minLength: 3
    });
    expect(js.validate("123")).toBeTruthy();
    expect(js.validate("12")).toBeFalsy();
    expect(js.validate("foo")).toBeFalsy();
});
