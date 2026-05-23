<template>
  <div class="chart-card card">
    <v-chart
      class="h-full w-full"
      :option="option"
      :loading="loading"
      autoresize
    />
  </div>
</template>

<script setup lang="ts">
import { use } from "echarts/core";
import { CanvasRenderer } from "echarts/renderers";
import { BarChart } from "echarts/charts";
import {
  TooltipComponent,
  LegendComponent,
} from "echarts/components";
import VChart, { THEME_KEY } from "vue-echarts";
import { computed, onMounted, onUnmounted, provide, ref } from "vue";
import { GridComponent } from "echarts/components";
import { ECBasicOption } from "echarts/types/dist/shared";
import { invoke } from "@tauri-apps/api/core";
import { useI18n } from "vue-i18n";

use([
  GridComponent,
  CanvasRenderer,
  BarChart,
  TooltipComponent,
  LegendComponent,
]);
const { t } = useI18n();
provide(THEME_KEY, "auto");
const loading = ref<boolean>(false);
const incomeData = ref<number[]>([0, 0, 0, 0, 0, 0, 0]);
const expenseData = ref<number[]>([0, 0, 0, 0, 0, 0, 0]);
const themeTick = ref(0);

const readToken = (name: string) => {
  themeTick.value;

  if (typeof window === "undefined") {
    return "";
  }

  return getComputedStyle(document.documentElement)
    .getPropertyValue(name)
    .trim();
};

const hslToken = (name: string) => `hsl(${readToken(name)})`;

const option = computed<ECBasicOption>(() => ({
  tooltip: {
    trigger: "axis",
    axisPointer: {
      type: "shadow",
    },
    backgroundColor: hslToken("--chart-tooltip"),
    borderColor: hslToken("--border"),
    borderWidth: 1,
    padding: [8, 10],
    textStyle: {
      color: hslToken("--chart-tooltip-foreground"),
      fontFamily: "Quicksand, sans-serif",
    },
    valueFormatter: (value: number | string) => {
      const numericValue = Number(value);
      return Number.isNaN(numericValue)
        ? String(value)
        : Math.abs(numericValue).toFixed(2);
    },
  },
  grid: {
    left: 8,
    right: 12,
    bottom: 8,
    top: 12,
    containLabel: true,
  },
  xAxis: [
    {
      type: "category",
      data: [
        t("mon"),
        t("tue"),
        t("wed"),
        t("thu"),
        t("fri"),
        t("sat"),
        t("sun"),
      ],
      axisTick: {
        alignWithLabel: true,
      },
      axisLine: {
        lineStyle: {
          color: hslToken("--chart-axis"),
        },
      },
      axisLabel: {
        color: hslToken("--muted-foreground"),
        fontFamily: "Quicksand, sans-serif",
      },
    },
  ],
  yAxis: [
    {
      type: "value",
      splitLine: {
        lineStyle: {
          color: hslToken("--chart-grid"),
        },
      },
      axisLabel: {
        color: hslToken("--muted-foreground"),
        fontFamily: "Quicksand, sans-serif",
      },
    },
  ],
  series: [
    {
      name: t("expenses"),
      type: "bar",
      barWidth: "20%",
      data: expenseData.value.map((value) => value * -1),
      animationDelay: function (idx: number) {
        return idx * 10;
      },
      color: hslToken("--metric-expense"),
      itemStyle: {
        borderRadius: [4, 4, 0, 0],
      },
    },
    {
      name: t("income"),
      type: "bar",
      barWidth: "20%",
      data: incomeData.value,
      animationDelay: function (idx: number) {
        return idx * 10;
      },
      color: hslToken("--metric-income"),
      itemStyle: {
        borderRadius: [4, 4, 0, 0],
      },
    },
  ],
}));

let themeObserver: MutationObserver | undefined;

onMounted(() => {
  themeTick.value += 1;

  themeObserver = new MutationObserver(() => {
    themeTick.value += 1;
  });

  themeObserver.observe(document.documentElement, {
    attributes: true,
    attributeFilter: ["class", "style"],
  });
});

onUnmounted(() => {
  themeObserver?.disconnect();
});

loading.value = true;
invoke("get_weekly_income_expenses").then((res: any) => {
  loading.value = false;
  expenseData.value = res["expenses"] as number[];
  incomeData.value = res["income"] as number[];
});
</script>

<style scoped>
.chart-card {
  width: 100%;
  height: 18rem;
  padding: var(--space-md);
}
</style>
