import { EditorView } from '@codemirror/view';
import { HighlightStyle, syntaxHighlighting } from '@codemirror/language';
import { tags as t } from '@lezer/highlight';

const baseTheme = EditorView.theme({
	'&': {
		color: 'var(--foreground)',
		backgroundColor: 'transparent',
		fontFamily: 'var(--font-mono)',
		fontSize: '16px'
	},
	'.cm-content': {
		caretColor: 'var(--primary)'
	},
	'&.cm-focused .cm-cursor': {
		borderLeftColor: 'var(--primary)',
		borderLeftWidth: '2px'
	},
	'&.cm-focused .cm-selectionBackground, ::selection': {
		backgroundColor: 'var(--muted)',
		color: 'inherit'
	},
	'.cm-gutters': {
		backgroundColor: 'transparent',
		color: 'var(--muted-foreground)',
		borderRight: '1px solid var(--border)',
		fontFamily: 'var(--font-mono)'
	},
	'.cm-activeLineGutter': {
		color: 'var(--primary)',
		backgroundColor: 'transparent'
	},
	'.cm-activeLine': {
		backgroundColor: 'transparent'
	}
});

const syntaxTheme = HighlightStyle.define([
	{ tag: [t.keyword, t.modifier, t.controlKeyword], color: 'var(--syntax-keyword)' },
	{
		tag: [t.function(t.variableName), t.definition(t.variableName), t.name],
		color: 'var(--syntax-func)'
	},
	{ tag: [t.number, t.string, t.bool], color: 'var(--syntax-value)' },
	{ tag: [t.operator, t.arithmeticOperator, t.logicOperator], color: 'var(--syntax-operator)' },
	{ tag: t.comment, color: 'var(--syntax-comment)', fontStyle: 'italic' },
	{ tag: [t.punctuation, t.bracket, t.paren], color: 'var(--syntax-punct)' },
	{ tag: t.variableName, color: 'var(--foreground)' }
]);

export const shadcnCodeMirrorTheme = [baseTheme, syntaxHighlighting(syntaxTheme)];
