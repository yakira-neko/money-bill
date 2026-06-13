<template>
  <div class="h-72 card w-full">
    <v-chart class="h-full w-full" :option="option" :loading="loading" autoresize />
  </div>
</template>

<script setup lang="ts">
import { use } from "echarts/core";
import { CanvasRenderer } from "echarts/renderers";
import { BarChart } from "echarts/charts";
import {
  TitleComponent,
  TooltipComponent,
  LegendComponent,
  GridComponent,
} from "echarts/components";
import VChart from "vue-echarts";
import { ref, watch } from "vue";
import { ECBasicOption } from "echarts/types/dist/shared";
import { invoke } from "@tauri-apps/api/core";
import { useI18n } from "vue-i18n";
import { useAppTheme } from "../../../composables/useAppTheme";

use([
  GridComponent,
  CanvasRenderer,
  BarChart,
  TitleComponent,
  TooltipComponent,
  LegendComponent,
]);

const { t } = useI18n();
const { isDark } = useAppTheme();
const loading = ref<boolean>(false);

const option = ref<ECBasicOption>({
  tooltip: {
    trigger: "axis",
    axisPointer: { type: "shadow" },
    backgroundColor: "rgba(255, 255, 255, 0.95)",
    borderColor: "#e2e8f0",
    textStyle: { color: "#0f172a" },
  },
  grid: {
    left: "3%",
    right: "4%",
    bottom: "3%",
    top: "10%",
    containLabel: true,
  },
  xAxis: [
    {
      type: "category",
      data: [t("mon"), t("tue"), t("wed"), t("thu"), t("fri"), t("sat"), t("sun")],
      axisTick: { alignWithLabel: true },
      axisLine: { lineStyle: { color: "#cbd5e1" } },
      axisLabel: { color: "#475569" },
    },
  ],
  yAxis: [
    {
      type: "value",
      splitLine: { lineStyle: { color: "#e2e8f0" } },
      axisLabel: { color: "#475569" },
    },
  ],
  series: [
    {
      name: t("expenses"),
      type: "bar",
      barWidth: "20%",
      data: [0, 0, 0, 0, 0, 0, 0],
      animationDelay: (idx: number) => idx * 10,
      color: "#e11d48",
      itemStyle: { borderRadius: [4, 4, 0, 0] },
    },
    {
      name: t("income"),
      type: "bar",
      barWidth: "20%",
      data: [0, 0, 0, 0, 0, 0, 0],
      animationDelay: (idx: number) => idx * 10,
      color: "#059669",
      itemStyle: { borderRadius: [4, 4, 0, 0] },
    },
  ],
});

// Apply theme-aware colors to the chart so it reads well in light & dark.
const applyChartTheme = (dark: boolean) => {
  const opt = option.value as any;
  const axisText = dark ? "#94a3b8" : "#475569";
  const gridLine = dark ? "#334155" : "#e2e8f0";
  const axisLine = dark ? "#475569" : "#cbd5e1";

  opt.tooltip.backgroundColor = dark ? "rgba(30, 41, 59, 0.95)" : "rgba(255, 255, 255, 0.95)";
  opt.tooltip.borderColor = dark ? "#334155" : "#e2e8f0";
  opt.tooltip.textStyle.color = dark ? "#f8fafc" : "#0f172a";

  opt.xAxis[0].axisLine.lineStyle.color = axisLine;
  opt.xAxis[0].axisLabel.color = axisText;
  opt.yAxis[0].splitLine.lineStyle.color = gridLine;
  opt.yAxis[0].axisLabel.color = axisText;

  opt.series[0].color = dark ? "#fb7185" : "#e11d48";
  opt.series[1].color = dark ? "#34d399" : "#059669";

  // Trigger reactivity for the deep mutation.
  option.value = { ...opt };
};

watch(isDark, applyChartTheme, { immediate: true });

loading.value = true;
invoke("get_weekly_income_expenses")
  .then((res: any) => {
    loading.value = false;
    const opt = option.value as any;
    opt.series[0].data = (res["expenses"] as number[]).map((v) => v * -1);
    opt.series[1].data = res["income"];
    option.value = { ...opt };
  })
  .catch(() => {
    loading.value = false;
  });
</script>
