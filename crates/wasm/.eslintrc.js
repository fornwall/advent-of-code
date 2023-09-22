module.exports = {
  env: {
    browser: true,
    es2023: true,
  },
  parserOptions: {
    sourceType: "module",
  },
  rules: {
    "require-jsdoc": 0,
    "max-len": ["error", { code: 140 }],
  },
  extends: ["prettier"],
};
