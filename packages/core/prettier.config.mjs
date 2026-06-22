import sharedConfig from '../../prettier.config.mjs';

export default {
	...sharedConfig,
	plugins: [...sharedConfig.plugins]
};
