import { EditorView } from '@codemirror/view';

const baseStyles = {
	'&': {
		color: 'hsl(var(--foreground))',
		backgroundColor: 'transparent'
	},
	'.cm-gutters': {
		backgroundColor: 'transparent',
		color: 'hsl(var(--muted-foreground))',
		borderRight: '1px solid hsl(var(--border))'
	},
	'.cm-activeLine': {
		backgroundColor: 'hsl(var(--muted) / 0.5)'
	},
	'.cm-activeLineGutter': {
		backgroundColor: 'transparent',
		color: 'hsl(var(--foreground))',
		fontWeight: 'bold'
	},
	'.cm-cursor': {
		borderLeftColor: 'hsl(var(--foreground))'
	},
	'&.cm-focused .cm-selectionBackground, .cm-selectionBackground, .cm-content ::selection': {
		backgroundColor: 'hsl(var(--primary) / 0.2)'
	}
};

export const janetLightTheme = EditorView.theme(baseStyles, { dark: false });
export const janetDarkTheme = EditorView.theme(baseStyles, { dark: true });
