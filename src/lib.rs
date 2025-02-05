mod plugin;

use plugin:GainPlugin;
use vst::plugin_main;

plugin_main!(GainPlugin);
