<script lang="ts">
	import { untrack } from 'svelte';
	import { EditorState, Compartment, type Extension } from '@codemirror/state';
	import { EditorView, basicSetup } from 'codemirror';
	import { keymap } from '@codemirror/view';
	import { indentWithTab } from '@codemirror/commands';
	import { clojure } from '@nextjournal/lang-clojure';
	import { shadcnCodeMirrorTheme } from './cm-theme';
	import type { HTMLAttributes } from 'svelte/elements';
	import type { ClassValue } from 'clsx';
	import { cn, type WithoutChildren } from '#ui/utils';

	type Props = WithoutChildren<HTMLAttributes<HTMLDivElement>> & {
		class?: ClassValue;
		value?: string;
		extensions?: Extension[];
		readonly?: boolean;
		onchange?: (val: string) => void;
	};

	let {
		class: className,
		value = $bindable(''),
		extensions = [],
		readonly = false,
		onchange,
		...restProps
	}: Props = $props();

	let editorContainer: HTMLDivElement;
	let view = $state<EditorView>();
	const extensionsCompartment = new Compartment();

	const fontSizeTheme = EditorView.theme({
		'&': {
			fontSize: '16px'
		}
	});

	let activeExtensions = $derived([
		keymap.of([indentWithTab]),
		...extensions,
		fontSizeTheme,
		clojure(),
		shadcnCodeMirrorTheme,
		EditorState.readOnly.of(readonly)
	]);

	$effect(() => {
		if (!editorContainer) return;

		untrack(() => {
			const state = EditorState.create({
				doc: value,
				extensions: [
					basicSetup,
					extensionsCompartment.of(activeExtensions),
					EditorView.updateListener.of((update) => {
						if (update.docChanged) {
							const newVal = update.state.doc.toString();
							if (newVal !== value) {
								value = newVal;
								onchange?.(newVal);
							}
						}
					})
				]
			});

			view = new EditorView({
				state,
				parent: editorContainer
			});
		});

		return () => {
			view?.destroy();
		};
	});

	$effect(() => {
		if (view && value !== view.state.doc.toString()) {
			view.dispatch({
				changes: { from: 0, to: view.state.doc.length, insert: value }
			});
		}
	});

	$effect(() => {
		if (view) {
			view.dispatch({
				effects: extensionsCompartment.reconfigure(activeExtensions)
			});
		}
	});
</script>

<div
	bind:this={editorContainer}
	class={cn(
		'relative h-full w-full overflow-hidden text-left',
		'[&_.cm-editor]:h-full [&_.cm-editor]:bg-transparent [&_.cm-scroller]:h-full',
		className
	)}
	{...restProps}
></div>
